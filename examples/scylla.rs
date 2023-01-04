use std::sync::Arc;

use scylla::{Session, SessionBuilder};

#[macro_use]
extern crate orm_scylladb;
#[macro_use]
extern crate log;

#[derive(ScyllaDBQuery)]
struct Demo1 {
    id: i64,
    name: String,
}
impl Demo1 {
    pub fn table_name() -> String {
        "todo input database tabale name".into()
    }
}


#[tokio::main]
async fn main() -> common_uu::IResult {
    let uri = "127.0.0.1:9042";
    let session: Session = SessionBuilder::new().known_node(uri).build().await?;
    let r = Demo1::db_query(&Arc::new(session), "".to_string(), (), None).await?;
    Ok(())
}
