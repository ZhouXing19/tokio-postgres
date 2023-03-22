use tokio_postgres::{NoTls, Error};

// Using Tokio runtime for async behaviour: https://crates.io/crates/tokio
#[tokio::main]
async fn main() -> Result<(), Error>{
    // Get DB client and connection
    let (client, connection) = tokio_postgres::connect(
        "postgresql://root@crlMBP-C02FV2KKMD6TMTQ4.local:26257/defaultdb?sslmode=disable",
        NoTls,
    )
        .await?;

    // Spawn connection
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Connection error: {}", error);
        }
    });

    // Do the query
    let users = client.query("SELECT * FROM t", &[]).await?;

    // println!("{:#?}", users);
    let value: i64 = users[0].get(0);
    println!("{:#?}", value);

    Ok(())
}