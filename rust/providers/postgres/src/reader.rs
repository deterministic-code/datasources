use crate::bind::{param_types, to_postgres};
use crate::command::row_record;
use crate::connection::{map_err, PostgresConnection};
use crate::pg_value::PgValue;
use async_trait::async_trait;
use datasources::{DatasourceError, Reader, Result, Row, Value};
use futures_core::Stream;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_postgres::Client;

enum Ask {
    Next,
    NextResult,
    Shutdown,
}

enum Answer {
    Ready,
    Row(Option<Row>),
    More(bool),
    Err(String),
}

pub(crate) struct PostgresReader {
    ask: Mutex<Option<UnboundedSender<Ask>>>,
    task: Mutex<Option<JoinHandle<()>>>,
    answer: tokio::sync::Mutex<UnboundedReceiver<Answer>>,
    current: Mutex<Option<Row>>,
}

impl PostgresReader {
    pub(crate) async fn open(
        connection: &PostgresConnection,
        statements: Vec<String>,
        params: HashMap<String, Value>,
    ) -> Result<Self> {
        let client = {
            let mut slot = connection.lock_client().await?;
            slot.take()
                .ok_or_else(|| DatasourceError::message("Connection is not open"))?
        };
        let slot = connection.client.clone();
        let opened = connection.opened.clone();
        let ready = connection.ready.clone();
        let (ask_tx, ask_rx) = tokio::sync::mpsc::unbounded_channel();
        let (ans_tx, ans_rx) = tokio::sync::mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            let client = client;
            if let Err(error) = drive(&client, &statements, &params, ask_rx, &ans_tx).await {
                let _ = ans_tx.send(Answer::Err(error.to_string()));
            }
            let mut held = slot.lock().await;
            if *opened.lock().unwrap() {
                *held = Some(client);
            }
            drop(held);
            ready.notify_waiters();
        });
        let mut answer = ans_rx;
        match answer.recv().await {
            Some(Answer::Ready) => Ok(Self {
                ask: Mutex::new(Some(ask_tx)),
                task: Mutex::new(Some(task)),
                answer: tokio::sync::Mutex::new(answer),
                current: Mutex::new(None),
            }),
            Some(Answer::Err(error)) => {
                let _ = ask_tx.send(Ask::Shutdown);
                let _ = task.await;
                Err(DatasourceError::message(error))
            }
            _ => {
                let _ = task.await;
                Err(DatasourceError::message("Reader failed to open"))
            }
        }
    }

    async fn ask(&self, ask: Ask) -> Result<Answer> {
        let tx = self
            .ask
            .lock()
            .expect("reader")
            .as_ref()
            .cloned()
            .ok_or_else(|| DatasourceError::message("Reader is closed"))?;
        tx.send(ask)
            .map_err(|_| DatasourceError::message("Reader is closed"))?;
        self.answer
            .lock()
            .await
            .recv()
            .await
            .ok_or_else(|| DatasourceError::message("Reader is closed"))
    }
}

impl Drop for PostgresReader {
    fn drop(&mut self) {
        if let Some(tx) = self.ask.lock().expect("reader").take() {
            let _ = tx.send(Ask::Shutdown);
        }
        let _ = self.task.lock().expect("reader").take();
    }
}

#[async_trait]
impl Reader for PostgresReader {
    fn current(&self) -> Option<Row> {
        self.current.lock().expect("reader").clone()
    }

    async fn next(&self) -> Result<bool> {
        match self.ask(Ask::Next).await? {
            Answer::Row(row) => {
                let found = row.is_some();
                *self.current.lock().expect("reader") = row;
                Ok(found)
            }
            Answer::Err(error) => Err(DatasourceError::message(error)),
            _ => Err(DatasourceError::message("Reader failed")),
        }
    }

    async fn next_result(&self) -> Result<bool> {
        *self.current.lock().expect("reader") = None;
        match self.ask(Ask::NextResult).await? {
            Answer::More(more) => Ok(more),
            Answer::Err(error) => Err(DatasourceError::message(error)),
            _ => Err(DatasourceError::message("Reader failed")),
        }
    }
}

async fn drive(
    client: &Client,
    statements: &[String],
    params: &HashMap<String, Value>,
    mut ask_rx: UnboundedReceiver<Ask>,
    ans_tx: &UnboundedSender<Answer>,
) -> Result<()> {
    let _ = ans_tx.send(Answer::Ready);
    if statements.is_empty() {
        return idle(&mut ask_rx, ans_tx).await;
    }
    let mut index = 0;
    loop {
        let (text, values) = to_postgres(&statements[index], params);
        let types = param_types(&values);
        let bound: Vec<PgValue> = values.into_iter().map(PgValue).collect();
        let statement = client.prepare_typed(&text, &types).await.map_err(map_err)?;
        let stream = client
            .query_raw(&statement, bound.iter())
            .await
            .map_err(map_err)?;
        let mut stream = std::pin::pin!(stream);
        loop {
            match ask_rx.recv().await {
                Some(Ask::Next) => {
                    let row = pull(stream.as_mut()).await.map_err(map_err)?;
                    let _ = ans_tx.send(Answer::Row(row.as_ref().map(row_record)));
                }
                Some(Ask::NextResult) => break,
                _ => {
                    drain(stream.as_mut()).await.map_err(map_err)?;
                    return Ok(());
                }
            }
        }
        drain(stream.as_mut()).await.map_err(map_err)?;
        drop(stream);
        index += 1;
        if index >= statements.len() {
            let _ = ans_tx.send(Answer::More(false));
            return idle(&mut ask_rx, ans_tx).await;
        }
        let _ = ans_tx.send(Answer::More(true));
    }
}

async fn idle(ask_rx: &mut UnboundedReceiver<Ask>, ans_tx: &UnboundedSender<Answer>) -> Result<()> {
    loop {
        match ask_rx.recv().await {
            Some(Ask::Next) => {
                let _ = ans_tx.send(Answer::Row(None));
            }
            Some(Ask::NextResult) => {
                let _ = ans_tx.send(Answer::More(false));
            }
            _ => return Ok(()),
        }
    }
}

async fn pull<S, T, E>(mut stream: Pin<&mut S>) -> std::result::Result<Option<T>, E>
where
    S: Stream<Item = std::result::Result<T, E>> + ?Sized,
{
    std::future::poll_fn(|cx| stream.as_mut().poll_next(cx))
        .await
        .transpose()
}

async fn drain<S, T, E>(mut stream: Pin<&mut S>) -> std::result::Result<(), E>
where
    S: Stream<Item = std::result::Result<T, E>> + ?Sized,
{
    while pull(stream.as_mut()).await?.is_some() {}
    Ok(())
}
