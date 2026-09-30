use crate::bulk_command::BulkCommand;
use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::i_where::IWhere;
use crate::native::Native;
use crate::parameter::Parameter;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;

pub struct BatchCommandIBulkWriteStrategy;

#[async_trait]
impl BulkWriteStrategy for BatchCommandIBulkWriteStrategy {
    async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response> {
        send(repo, &repo.inserts(items)?).await
    }

    async fn update(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response> {
        send(repo, &repo.updates(items, by_where)?).await
    }

    async fn delete(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response> {
        send(repo, &repo.deletes(items, by_where)?).await
    }
}

async fn send(repo: &BulkCRUDRepository, commands: &[BulkCommand]) -> Result<Response> {
    if commands.is_empty() {
        return Ok(Response::new(Value::Null, 0, None));
    }
    let renamed: Vec<BulkCommand> = commands
        .iter()
        .enumerate()
        .map(|(index, command)| rename(command, index))
        .collect();
    let text = renamed
        .iter()
        .map(|command| command.text.as_str())
        .collect::<Vec<_>>()
        .join("; ");
    let parameters: Vec<Parameter> = renamed
        .iter()
        .flat_map(|command| command.parameters.clone())
        .collect();
    repo.command()
        .execute_native(&Native::new(text), Some(&parameters))
        .await
}

fn rename(command: &BulkCommand, index: usize) -> BulkCommand {
    let mut text = command.text.clone();
    let mut parameters = command.parameters.clone();
    let mut names: Vec<String> = parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    names.sort();
    names.dedup();
    names.sort_by_key(|name| std::cmp::Reverse(name.len()));
    for name in names {
        let alias = format!("{name}__{index}");
        text = rename_token(&text, &name, &alias);
        for parameter in &mut parameters {
            if parameter.name == name {
                parameter.name = alias.clone();
            }
        }
    }
    BulkCommand { text, parameters }
}

fn rename_token(text: &str, name: &str, alias: &str) -> String {
    let token = format!("${name}");
    let next = format!("${alias}");
    let mut result = String::new();
    let mut index = 0;
    while let Some(at) = text[index..].find(&token) {
        let at = index + at;
        let end = at + token.len();
        let boundary = match text[end..].chars().next() {
            None => true,
            Some(ch) => !ch.is_ascii_alphanumeric() && ch != '_',
        };
        result.push_str(&text[index..at]);
        result.push_str(if boundary { &next } else { &token });
        index = end;
    }
    result.push_str(&text[index..]);
    result
}
