#![allow(unused_imports)]

use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = DB.connect::<Ws>("127.0.0.1:8000").await {
        tracing::error!("Failed to connect to database: {:?}", e);
        return;
    }

    if let Err(e) = DB.use_ns("dx-surreal").use_db("dx-surreal").await {
        tracing::error!("Failed to select namespace and database: {:?}", e);
        return;
    }

    /*
    Commented out to simplify reproducing the issue.
    if let Err(e) = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        tracing::error!("Failed to sign in as root: {:?}", e);
        return;
    }*/

    tracing::info!("Connected. Signed in as root.");

    // will always fail
    if let Ok(data) = DB.query("INFO FOR SCOPE users;").await {
        tracing::info!("Data: {:?}", data);
    }
    if let Ok(data) = DB.query("INFO FOR NS;").await {
        tracing::info!("Data: {:?}", data);
    }
}
