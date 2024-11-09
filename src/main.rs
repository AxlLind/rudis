use std::net::TcpListener;
use std::io::{BufReader, BufWriter, Write};
use anyhow::{self, Context};

use redis_in_rust::{Database, Parser, execute_command, write_response};

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8888))?;
    let mut db = Database::default();
    for stream in listener.incoming() {
        let stream = stream?;
        let mut parser = Parser::new(BufReader::new(stream.try_clone()?));
        let mut writer = BufWriter::new(stream);
        loop {
            let r = match parser.read_command() {
                Ok(cmd) => {
                    println!("Got command: {}", cmd);
                    execute_command(&mut db, cmd)
                },
                Err(e) => Err(e),
            };
            let e = match r {
                Ok(res) => write_response(&mut writer, res),
                Err(e) => write!(writer, "-ERR {e}\r\n").with_context(|| "failed to write error"),
            };
            if let Err(e) = e {
                println!("Client error: {e}");
                break;
            }
            if let Err(e) = writer.flush() {
                println!("Client error: {e}");
                break;
            }
        }
    }
    Ok(())
}
