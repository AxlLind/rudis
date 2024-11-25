use std::io::Write;
use anyhow::{self, Context};
use macro_rules_attribute::apply;
use smol_macros::main;
use smol::channel::{Receiver, Sender};
use smol::io::{AsyncRead, AsyncWriteExt, BufReader};
use smol::net::{TcpListener, TcpStream};

use rudis::{execute_command, write_response, Command, Database, Response};

mod cmd_parser;
use cmd_parser::Parser;

async fn read_command_task<R: AsyncRead + Unpin>(
    mut parser: Parser<R>,
    db_tx: Sender<(Sender<anyhow::Result<Response>>, Command)>,
    tx: Sender<anyhow::Result<Response>>,
) -> anyhow::Result<()> {
    loop {
        match parser.read_command().await {
            Ok(cmd) => {
                println!("Got command: {}", cmd);
                db_tx.send((tx.clone(), cmd)).await?;
            }
            Err(e) => tx.send(Err(e)).await?,
        }
    }
}

async fn send_response_task(mut stream: TcpStream, rx: Receiver<anyhow::Result<Response>>) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match rx.recv().await? {
            Ok(res) => write_response(&mut buf, res),
            Err(e) => write!(&mut buf, "-ERR {e}\r\n").with_context(|| "failed to write error"),
        }.unwrap();
        stream.write_all(&buf).await?;
    }
}

async fn handle_connection(stream: TcpStream, db_tx: Sender<(Sender<anyhow::Result<Response>>, Command)>) {
    let parser = Parser::new(BufReader::new(stream.clone()));
    let (tx, rx) = smol::channel::bounded(128);
    let _ = smol::future::zip(
        read_command_task(parser, db_tx, tx),
        send_response_task(stream, rx),
    ).await;
}

async fn database_task(db_rx: Receiver<(Sender<anyhow::Result<Response>>, Command)>) -> anyhow::Result<()> {
    let mut db = Database::default();
    loop {
        let (tx, cmd) = db_rx.recv().await?;
        let res = execute_command(&mut db, cmd);
        tx.send(res).await?;
    }
}

#[apply(main!)]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8888)).await?;
    let (db_tx, db_rx) = smol::channel::bounded(1024);
    smol::spawn(database_task(db_rx)).detach();
    loop {
        let (stream, _) = listener.accept().await?;
        smol::spawn(handle_connection(stream, db_tx.clone())).detach();
    }
}
