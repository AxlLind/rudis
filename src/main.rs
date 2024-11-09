use std::net::TcpListener;
use std::io::{BufReader, BufWriter, Write};
use anyhow::{self, Context};

use redis_in_rust::{Database, Response, Parser, execute_command};

fn write_response(writer: &mut impl Write, res: Response) -> anyhow::Result<()> {
    match res {
        Response::String(value) => {
            writer.write_all(b"+")?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Response::Number(value) => {
            write!(writer, ":{value}\r\n")?;
        },
        Response::Array(value) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &value {
                write!(writer, "${}\r\n", v.len())?;
                writer.write_all(v)?;
                write!(writer, "\r\n")?;
            }
        }
        Response::CommandList(mut value) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &mut value {
                write!(writer, "${}\r\n", v.name.len())?;
                writer.write_all(v.name)?;
                write_response(writer, Response::Number(v.arity))?;
                write_response(writer, Response::Array(v.flags.iter().map(|s| s.to_vec()).collect()))?;
                write_response(writer, Response::Number(v.first_key))?;
                write_response(writer, Response::Number(v.last_key))?;
                write_response(writer, Response::Number(v.step))?;
            }
        },
        Response::Nil => write!(writer, "$-1\r\n")?,
    }
    Ok(())
}

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
