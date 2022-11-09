use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct SqliteConnector {
    pub pool: SqlitePool,
}

static INSTANCE: OnceCell<Arc<SqliteConnector>> = OnceCell::new();

impl SqliteConnector {
    pub async fn new(pool: Result<SqlitePool>) -> Result<Arc<Self>> {
        match INSTANCE.get() {
            Some(x) => return Ok(x.clone()),
            None => match pool {
                Ok(x) => {
                    let ret = Arc::new(Self { pool: x });
                    INSTANCE.set(ret.clone()).expect("failed to set singleton");
                    return Ok(ret);
                }
                Err(e) => return Err(e.into()),
            },
        }
    }
}
