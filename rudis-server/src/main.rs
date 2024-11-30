use std::io::Write;
use clap::Parser;
use macro_rules_attribute::apply;
use smol_macros::main;
use smol::channel::{Receiver, Sender};
use smol::io::{AsyncWriteExt, BufReader};
use smol::net::{TcpListener, TcpStream};

use rudis::{execute_command, write_response, Command, Database, Response};

mod cmd_parser;
mod async_pipe;
use cmd_parser::CmdParser;
use async_pipe::AsyncPipe;

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    /// port to listen to
    #[arg(short, long, default_value = "8888")]
    port: u16,

    /// ip to bind to
    #[arg(long, default_value = "0.0.0.0")]
    bind: String,
}

async fn read_command_task(
    stream: TcpStream,
    pipe: AsyncPipe<Command, Response>,
    tx: Sender<anyhow::Result<Response>>,
) -> anyhow::Result<()> {
    let mut parser = CmdParser::new(BufReader::new(stream));
    loop {
        match parser.read_command().await {
            Ok(cmd) => {
                println!("Got command: {}", cmd);
                pipe.send(cmd, tx.clone()).await;
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

async fn handle_connection(stream: TcpStream, pipe: AsyncPipe<Command, Response>) {
    let (tx, rx) = smol::channel::bounded(128);
    let _ = smol::future::zip(
        read_command_task(stream.clone(), pipe, tx),
        send_response_task(stream, rx),
    ).await;
}

async fn database_task(pipe: AsyncPipe<Command, Response>) {
    let mut db = Database::default();
    loop {
        let (cmd, tx) = pipe.recv().await;
        let res = execute_command(&mut db, cmd);
        let _ = tx.send(res).await;
    }
}

async fn run_server(bind: &str, port: u16) -> anyhow::Result<()> {
    let listener = TcpListener::bind((bind, port)).await?;
    let pipe = AsyncPipe::new(1024);
    smol::spawn(database_task(pipe.clone())).detach();
    loop {
        let (stream, _) = listener.accept().await?;
        smol::spawn(handle_connection(stream, pipe.clone())).detach();
    }
}

#[apply(main!)]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    run_server(&args.bind, args.port).await
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use smol::io::AsyncBufReadExt;
    use smol_macros::test;

    async fn send_cmd(stream: &mut TcpStream, cmd: &[&str]) {
        let mut buf = Vec::new();
        write!(buf, "*{}\r\n", cmd.len()).unwrap();
        for c in cmd {
            write!(buf, "${}\r\n{}\r\n", c.len(), c).unwrap();
        }
        stream.write_all(&buf).await.unwrap();
    }

    async fn read_resp(reader: &mut BufReader<TcpStream>) -> Vec<u8> {
        let mut buf = Vec::new();
        reader.read_until(b'\n', &mut buf).await.unwrap();
        buf
    }

    #[apply(test!)]
    async fn test_server_communication() {
        smol::spawn(run_server("127.0.0.1", 61111)).detach();
        smol::Timer::after(Duration::from_millis(100)).await;
        let mut stream = TcpStream::connect(("127.0.0.1", 61111)).await.unwrap();
        send_cmd(&mut stream, &["set", "x", "123"]).await;
        send_cmd(&mut stream, &["get", "x"]).await;
        send_cmd(&mut stream, &["get", "y"]).await;
        let mut reader = BufReader::new(stream);
        assert_eq!(read_resp(&mut reader).await, b"+OK\r\n");
        assert_eq!(read_resp(&mut reader).await, b"+123\r\n");
        assert_eq!(read_resp(&mut reader).await, b"$-1\r\n");
    }
}
