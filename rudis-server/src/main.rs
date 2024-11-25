use std::io::Write;
use macro_rules_attribute::apply;
use smol_macros::main;
use smol::channel::{Receiver, Sender};
use smol::io::{AsyncWriteExt, BufReader};
use smol::net::{TcpListener, TcpStream};

use rudis::{execute_command, write_response, Command, Database, Response};

mod cmd_parser;
use cmd_parser::Parser;

async fn read_command_task(
    mut parser: Parser<BufReader<TcpStream>>,
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
    let mut buf = Vec::with_capacity(1 << 16);
    loop {
        buf.clear();
        match rx.recv().await? {
            Ok(res) => write_response(&mut buf, res)?,
            Err(e) => write!(&mut buf, "-ERR {e}\r\n")?,
        };
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

async fn database_task(rx: Receiver<(Sender<anyhow::Result<Response>>, Command)>) -> anyhow::Result<()> {
    let mut db = Database::default();
    loop {
        let (tx, cmd) = rx.recv().await?;
        let res = execute_command(&mut db, cmd);
        tx.send(res).await?;
    }
}

#[apply(main!)]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8888)).await?;
    let (tx, rx) = smol::channel::bounded(1024);
    smol::spawn(database_task(rx)).detach();
    loop {
        let (stream, _) = listener.accept().await?;
        smol::spawn(handle_connection(stream, tx.clone())).detach();
    }
}
