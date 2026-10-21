use crate::connection::connect_url;
use tokio_postgres::NoTls;
use uuid::Uuid;

pub struct Isolated {
    pub url: String,
    name: String,
    base: String,
}

fn replace_db(url: &str, name: &str) -> String {
    match url.rfind('/') {
        Some(index) if url[..index].contains("://") => {
            let prefix = &url[..=index];
            match url[index + 1..].find('?') {
                Some(q) => format!("{prefix}{name}{}", &url[index + 1 + q..]),
                None => format!("{prefix}{name}"),
            }
        }
        _ => url.to_string(),
    }
}

async fn admin(url: &str) -> tokio_postgres::Client {
    let (client, connection) = tokio_postgres::connect(&connect_url(url), NoTls)
        .await
        .unwrap();
    tokio::spawn(async move {
        let _ = connection.await;
    });
    client
}

pub async fn create() -> Isolated {
    let base = std::env::var("POSTGRES_URL").expect("POSTGRES_URL");
    let name = format!("ds_{}", Uuid::new_v4().simple());
    admin(&base)
        .await
        .batch_execute(&format!("CREATE DATABASE {name}"))
        .await
        .unwrap();
    Isolated {
        url: replace_db(&base, &name),
        name,
        base,
    }
}

impl Isolated {
    pub async fn drop(self) {
        let client = admin(&self.base).await;
        client
            .batch_execute(&format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid()",
                self.name
            ))
            .await
            .ok();
        client
            .batch_execute(&format!("DROP DATABASE {}", self.name))
            .await
            .ok();
    }
}
