use std::collections::{HashMap, HashSet};
use std::net::TcpListener;
use std::io::{BufReader, BufWriter, Write};
use anyhow;

mod command;
use command::Parser;

#[allow(unused)] // TODO: remove this
#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Hash(HashMap<Vec<u8>, Vec<u8>>),
    Set(HashSet<Vec<u8>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Response {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Number(i64),
    Nil,
}

fn escape_bytes(bytes: &[u8]) -> String {
    bytes.iter().flat_map(|&b| std::ascii::escape_default(b)).map(|b| b as char).collect()
}

fn int_from_bytes(bytes: &[u8]) -> anyhow::Result<i64> {
    std::str::from_utf8(bytes)
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-utf8 value"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-numeric value"))
}

fn incr_by(state: &mut HashMap<Vec<u8>, Value>, key: Vec<u8>, step: i64) -> anyhow::Result<Response> {
    let val = step + match state.get(&key) {
        Some(Value::String(v)) => int_from_bytes(v)?,
        Some(_) => anyhow::bail!("INCR on non-string value"),
        None => 0,
    };
    state.insert(key, Value::String(val.to_string().into_bytes()));
    Ok(Response::Number(val))
}

fn execute_command(state: &mut HashMap<Vec<u8>, Value>, cmd: Vec<Vec<u8>>) -> anyhow::Result<Response> {
    println!("Command: {:?}", cmd.iter().map(|s| escape_bytes(s)).collect::<Vec<_>>());
    let value = match cmd[0].as_slice() {
        b"APPEND" => {
            let [_, key, value] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected APPEND key value"))?;
            let len = match state.get_mut(&key) {
                Some(Value::String(v)) => {
                    v.extend(value);
                    v.len()
                },
                Some(_) => anyhow::bail!("GET on non-string value"),
                None => {
                    let len = value.len();
                    state.insert(key, Value::String(value));
                    len
                },
            };
            Response::Number(len as _)
        }
        b"COPY" => {
            let [_, src, dst] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected COPY src dst"))?;
            match state.get(&src) {
                Some(v) => {
                    state.insert(dst, v.clone());
                    Response::Number(1)
                }
                None => Response::Number(0),
            }
        }
        b"DECR" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected DECR key"))?;
            incr_by(state, key, -1)?
        }
        b"DECRBY" => {
            let [_, key, step] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected DECRBY key step"))?;
            let step = int_from_bytes(&step).map_err(|_| anyhow::anyhow!("Invalid step in DECRBY"))?;
            incr_by(state, key, -step)?
        }
        b"DEL" => {
            let removed = cmd[1..].iter().filter(|&key| state.remove(key).is_some()).count();
            Response::Number(removed as _)
        }
        b"EXISTS" => {
            Response::Number(cmd[1..].iter().filter(|&key| state.contains_key(key)).count() as _)
        }
        b"GET" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected GET key"))?;
            match state.get(&key) {
                Some(Value::String(v)) => Response::String(v.clone()),
                Some(_) => anyhow::bail!("GET on non-string value"),
                None => Response::Nil,
            }
        }
        b"GETDEL" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected GETDEL key"))?;
            match state.get(&key) {
                Some(Value::String(v)) => {
                    let val = Response::String(v.clone());
                    state.remove(&key);
                    val
                }
                Some(_) => anyhow::bail!("GETDEL on non-string value"),
                None => Response::Nil,
            }
        }
        b"GETSET" => {
            let [_, key, value] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected GETSET key value"))?;
            match state.get(&key) {
                Some(Value::String(v)) => {
                    let val = Response::String(v.clone());
                    state.insert(key, Value::String(value));
                    val
                }
                Some(_) => anyhow::bail!("GETSET on non-string value"),
                None => Response::Nil,
            }
        }
        b"INCR" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected INCR key"))?;
            incr_by(state, key, 1)?
        }
        b"INCRBY" => {
            let [_, key, step] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected INCRBY key step"))?;
            let step = int_from_bytes(&step).map_err(|_| anyhow::anyhow!("Invalid step in INCRBY"))?;
            incr_by(state, key, step)?
        }
        b"RENAME" => {
            let [_, key, newkey] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected RENAME key newkey"))?;
            let val = state.remove(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
            state.insert(newkey, val);
            Response::String(b"OK".to_vec())
        }
        b"SET" => {
            let [_, key, value] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected SET key value"))?;
            state.insert(key, Value::String(value));
            Response::String(b"OK".to_vec())
        }
        b"STRLEN" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected STRLEN key"))?;
            match state.get(&key) {
                Some(Value::String(v)) => Response::Number(v.len() as _),
                Some(_) => anyhow::bail!("STRLEN on non-string value"),
                None => Response::Number(0),
            }
        }
        b"TYPE" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected STRLEN key"))?;
            let t: &[u8] = match state.get(&key) {
                Some(Value::String(_)) => b"string",
                Some(Value::List(_)) => b"list",
                Some(Value::Hash(_)) => b"hash",
                Some(Value::Set(_)) => b"set",
                None => b"none",
            };
            Response::String(t.to_vec())
        }
        b"COMMAND" => {
            // TODO: Implement this somehow
            Response::List(vec![])
        }
        _ => anyhow::bail!("Unrecognized command: {:?}", escape_bytes(&cmd[0])),
    };
    Ok(value)
}

fn write_response(writer: &mut impl Write, res: anyhow::Result<Response>) -> anyhow::Result<()> {
    match res {
        Ok(Response::String(value)) => {
            writer.write_all(b"+")?;
            writer.write_all(&value)?;
            writer.write_all(b"\r\n")?;
        }
        Ok(Response::List(value)) => {
            write!(writer, "*{}\r\n", value.len())?;
            for v in &value {
                write!(writer, "${}\r\n", v.len())?;
                writer.write_all(v)?;
                write!(writer, "\r\n")?;
            }
        }
        Ok(Response::Number(value)) => write!(writer, ":{value}\r\n")?,
        Ok(Response::Nil) => write!(writer, "$-1\r\n")?,
        Err(e) => write!(writer, "-ERR {e}\r\n")?,
    }
    writer.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8888))?;
    let mut state = HashMap::new();
    for stream in listener.incoming() {
        let stream = stream?;
        let mut parser = Parser::new(BufReader::new(stream.try_clone()?));
        let mut writer = BufWriter::new(stream);
        loop {
            let r = match parser.read_string_array() {
                Ok(cmd) => execute_command(&mut state, cmd),
                Err(e) => Err(e),
            };
            if let Err(e) = write_response(&mut writer, r) {
                println!("Client error: {e}");
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! exec_cmd {
        ($state:expr, $($cmd:expr),+) => {
            execute_command(&mut $state, vec![$($cmd.as_bytes().to_vec(),)+]).unwrap()
        }
    }

    #[test]
    fn test_get_set() {
        let mut state = HashMap::new();
        assert_eq!(exec_cmd!(state, "GET", "a"), Response::Nil);

        assert_eq!(exec_cmd!(state, "SET", "a", "b"), Response::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(state, "GET", "a"), Response::String(b"b".to_vec()));

        assert_eq!(exec_cmd!(state, "SET", "a", "c"), Response::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(state, "GET", "a"), Response::String(b"c".to_vec()));

        assert_eq!(exec_cmd!(state, "DEL", "a"), Response::Number(1));
        assert_eq!(exec_cmd!(state, "DEL", "b"), Response::Number(0));
        assert_eq!(exec_cmd!(state, "DEL", "a", "b", "c"), Response::Number(0));
    }
}
