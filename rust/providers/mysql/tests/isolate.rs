use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub async fn create() -> Isolated {
    let base = std::env::var("MYSQL_URL").expect("MYSQL_URL");
    let name = format!(
        "ds_{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let mut admin = Conn::from_url(base.clone()).await.unwrap();
    admin
        .query_drop(format!("CREATE DATABASE `{name}`"))
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
        let mut admin = Conn::from_url(self.base).await.unwrap();
        admin
            .query_drop(format!("DROP DATABASE `{}`", self.name))
            .await
            .ok();
    }
}
