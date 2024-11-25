use anyhow::{self, Context};

use std::io::Write;
use rudis::{Database, execute_command, write_response};
use smol::net::{TcpListener, TcpStream};
use smol::io::{AsyncWriteExt, BufReader};

mod cmd_parser;
use cmd_parser::Parser;

async fn handle_connection(stream: TcpStream) -> anyhow::Result<()> {
    let mut parser = Parser::new(BufReader::new(stream.clone()));
    let mut writer = stream;
    let mut db = Database::default();
    let mut buf = Vec::new();
    loop {
        let r = match parser.read_command().await {
            Ok(cmd) => {
                println!("Got command: {}", cmd);
                if cmd.cmd() == "quit" {
                    break;
                }
                execute_command(&mut db, cmd)
            },
            Err(e) => Err(e),
        };
        buf.clear();
        match r {
            Ok(res) => write_response(&mut buf, res),
            Err(e) => write!(&mut buf, "-ERR {e}\r\n").with_context(|| "failed to write error"),
        }.unwrap();
        writer.write_all(&buf).await?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let listener = TcpListener::bind(("0.0.0.0", 8888)).await?;
        loop {
            let (stream, _addr) = listener.accept().await?;
            smol::spawn(handle_connection(stream)).detach();
        }
    })
}
