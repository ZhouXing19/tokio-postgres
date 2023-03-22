use tokio_postgres::{NoTls, Error, Client, Connection, Socket};
use std::{thread, time};
use std::ptr::null;
use tokio_postgres::tls::NoTlsStream;

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

    call_query_no_for_loop(client).await;
    // ---- output ----
    // [call_query_no_for_loop] # prepared_stmts:4
    // name: s2, stmt: PREPARE s2 AS SELECT * FROM t
    // name: s3, stmt: PREPARE s3 AS SELECT * FROM pg_catalog.pg_prepared_statements
    // name: s0, stmt: PREPARE s0 AS SELECT * FROM t
    // name: s1, stmt: PREPARE s1 AS SELECT * FROM t


    //  call_query_for_loop(client).await;
    // ---- output ----
    // [call_query_for_loop] # prepared_stmts:1
    // name: s2, stmt: PREPARE s2 AS SELECT * FROM pg_catalog.pg_prepared_statements

    // call_exec_no_for_loop(client).await;
    // ---- output ----
    //[call_exec_no_for_loop] # prepared_stmts:1
    // name: s0, stmt: PREPARE s0 AS SELECT * FROM pg_catalog.pg_prepared_statements

    // call_exec_for_loop(client).await;
    // ---- output ----
    // [call_exec_for_loop] # prepared_stmts:1
    // name: s0, stmt: PREPARE s0 AS SELECT * FROM pg_catalog.pg_prepared_statements


    Ok(())
}

async fn call_query_no_for_loop(client: Client) -> Result<(), Error> {
        let res = client.query("SELECT * FROM t", &[]).await?;
        let res = client.query("SELECT * FROM t", &[]).await?;
        let res = client.query("SELECT * FROM t", &[]).await?;

        // Do the query
        let users = client.query("SELECT * FROM pg_catalog.pg_prepared_statements", &[]).await?;
        println!("[call_query_no_for_loop] # prepared_stmts:{:#?}", users.len());
        for user in users.into_iter() {
            let name: &str = user.get(0);
            let stmt: &str = user.get(1);
            match name {
                _ => println!("name: {name}, stmt: {stmt}", name=name, stmt=stmt),
            }
        }

    Ok(())
}

async fn call_query_for_loop(client: Client) -> Result<(), Error> {
    for i in 1..3 {
        let res = client.query("SELECT * FROM t", &[]).await?;
    }

    // Do the query
    let users = client.query("SELECT * FROM pg_catalog.pg_prepared_statements", &[]).await?;
    println!("[call_query_for_loop] # prepared_stmts:{:#?}", users.len());
    for user in users.into_iter() {
        let name: &str = user.get(0);
        let stmt: &str = user.get(1);
        match name {
            _ => println!("name: {name}, stmt: {stmt}", name=name, stmt=stmt),
        }
    }
    Ok(())
}

async fn call_exec_no_for_loop(client: Client) -> Result<(), Error> {
    let res = client.execute_raw("INSERT INTO t VALUES ($1, pg_sleep(2)), ($2, pg_sleep(1)), ($3, pg_sleep(1))", &[1i64,2i64,3i64]);
    let res = client.execute_raw("INSERT INTO t VALUES ($1, pg_sleep(2)), ($2, pg_sleep(1)), ($3, pg_sleep(1))", &[1i64,2i64,3i64]);
    let res = client.execute_raw("INSERT INTO t VALUES ($1, pg_sleep(2)), ($2, pg_sleep(1)), ($3, pg_sleep(1))", &[1i64,2i64,3i64]);

    // Do the query
    let users = client.query("SELECT * FROM pg_catalog.pg_prepared_statements", &[]).await?;
    println!("[call_exec_no_for_loop] # prepared_stmts:{:#?}", users.len());
    for user in users.into_iter() {
        let name: &str = user.get(0);
        let stmt: &str = user.get(1);
        match name {
            _ => println!("name: {name}, stmt: {stmt}", name=name, stmt=stmt),
        }
    }

    Ok(())
}

async fn call_exec_for_loop(client: Client) -> Result<(), Error> {
    for i in 1..3 {
        let res = client.execute_raw("INSERT INTO t VALUES ($1, pg_sleep(2)), ($2, pg_sleep(1)), ($3, pg_sleep(1))", &[1i64,2i64,3i64]);
    }

    // Do the query
    let users = client.query("SELECT * FROM pg_catalog.pg_prepared_statements", &[]).await?;
    println!("[call_exec_for_loop] # prepared_stmts:{:#?}", users.len());
    for user in users.into_iter() {
        let name: &str = user.get(0);
        let stmt: &str = user.get(1);
        match name {
            _ => println!("name: {name}, stmt: {stmt}", name=name, stmt=stmt),
        }
    }

    Ok(())
}