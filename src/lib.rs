use std::collections::{HashMap, HashSet};
use anyhow;

mod command;
pub use command::{Parser, Command};

#[allow(unused)] // TODO: remove this
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Hash(HashMap<Vec<u8>, Vec<u8>>),
    Set(HashSet<Vec<u8>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    String(Vec<u8>),
    List(Vec<Vec<u8>>),
    Number(i64),
    Nil,
}

pub struct Database {
    state: HashMap<Vec<u8>, Value>
}

impl Database {
    pub fn new() -> Self {
        Self { state: HashMap::new() }
    }

    pub fn get(&mut self, key: &[u8]) -> Option<&mut Value> {
        self.state.get_mut(key)
    }

    pub fn get_str(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut Vec<u8>>> {
        match self.get(key) {
            Some(Value::String(s)) => Ok(Some(s)),
            Some(_) => anyhow::bail!("expected string value"),
            None => Ok(None)
        }
    }

    pub fn get_list(&mut self, key: &[u8]) -> anyhow::Result<Option<&mut Vec<Vec<u8>>>> {
        match self.get(key) {
            Some(Value::List(v)) => Ok(Some(v)),
            Some(_) => anyhow::bail!("expected list value"),
            None => Ok(None)
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Value) -> Option<Value> {
        self.state.insert(key, value)
    }

    pub fn del(&mut self, key: &[u8]) -> Option<Value> {
        self.state.remove(key)
    }

    pub fn is_set(&self, key: &[u8]) -> bool {
        self.state.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
}

pub fn escape_bytes(bytes: &[u8]) -> String {
    bytes.iter().flat_map(|&b| std::ascii::escape_default(b)).map(|b| b as char).collect()
}

fn int_from_bytes(bytes: &[u8]) -> anyhow::Result<i64> {
    std::str::from_utf8(bytes)
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-utf8 value"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-numeric value"))
}

fn incr_by(db: &mut Database, key: Vec<u8>, step: i64) -> anyhow::Result<Response> {
    let val = step + match db.get_str(&key)? {
        Some(v) => int_from_bytes(v)?,
        None => 0,
    };
    db.set(key, Value::String(val.to_string().into_bytes()));
    Ok(Response::Number(val))
}

pub fn execute_command(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let value = match cmd.cmd() {
        "APPEND" => {
            let (key, value) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            let len = match db.get_str(&key)? {
                Some(v) => {
                    v.extend(value);
                    v.len()
                }
                None => {
                    let len = value.len();
                    db.set(key, Value::String(value));
                    len
                }
            };
            Response::Number(len as _)
        }
        "COPY" => {
            let (src, dst) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            match db.get(&src) {
                Some(v) => {
                    let copy = v.clone();
                    db.set(dst, copy);
                    Response::Number(1)
                }
                None => Response::Number(0),
            }
        }
        "DECR" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            incr_by(db, key, -1)?
        }
        "DECRBY" => {
            let (key, step) = cmd.parse_args::<(Vec<u8>, i64)>()?;
            incr_by(db, key, -step)?
        }
        "DEL" => {
            let keys = cmd.parse_args::<Vec<Vec<u8>>>()?;
            anyhow::ensure!(!keys.is_empty(), "expected DEL key [key ...]");
            Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _)
        }
        "EXISTS" => {
            let keys = cmd.parse_args::<Vec<Vec<u8>>>()?;
            anyhow::ensure!(!keys.is_empty(), "expected EXISTS key [key ...]");
            Response::Number(keys.iter().filter(|&key| db.is_set(key)).count() as _)
        }
        "FLUSHALL" => {
            let _mode = cmd.parse_args::<Option<Vec<u8>>>()?;
            db.clear();
            Response::String(b"OK".to_vec())
        }
        "GET" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            match db.get_str(&key)? {
                Some(s) => Response::String(s.clone()),
                None => Response::Nil,
            }
        }
        "GETDEL" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            match db.get_str(&key)? {
                Some(s) => {
                    let val = Response::String(std::mem::take(s));
                    db.del(&key);
                    val
                }
                None => Response::Nil,
            }
        }
        "GETSET" => {
            let (key, value) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            match db.get_str(&key)? {
                Some(s) => Response::String(std::mem::replace(s, value)),
                None => Response::Nil,
            }
        }
        "INCR" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            incr_by(db, key, 1)?
        }
        "INCRBY" => {
            let (key, step) = cmd.parse_args::<(Vec<u8>, i64)>()?;
            incr_by(db, key, step)?
        }
        "LPUSH" => {
            let (key, elements) = cmd.parse_args::<(Vec<u8>, Vec<Vec<u8>>)>()?;
            anyhow::ensure!(!elements.is_empty(), "expected LPUSH key element [element ...]");
            match db.get_list(&key)? {
                Some(list) => {
                    for (i, e) in elements.into_iter().enumerate() {
                        list.insert(i, e);
                    }
                    Response::Number(list.len() as _)
                }
                None => {
                    let len = elements.len();
                    db.set(key, Value::List(elements));
                    Response::Number(len as _)
                }
            }
        }
        "LRANGE" => {
            let (key, start, stop) = cmd.parse_args::<(Vec<u8>, i64, i64)>()?;
            match db.get_list(&key)? {
                Some(list) => {
                    let start = if start < 0 {list.len() - 2 - start as usize} else {start as usize};
                    let stop = if stop < 0 {list.len() - 2 - stop as usize} else {stop as usize};
                    // TODO: Implement more correct index handling here
                    Response::List(list[start..=stop].iter().cloned().collect())
                }
                None => Response::List(Vec::new()),
            }
        }
        "RENAME" => {
            let (key, newkey) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
            db.set(newkey, val);
            Response::String(b"OK".to_vec())
        }
        "RENAMENX" => {
            let (key, newkey) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
            if db.is_set(&newkey) {
                Response::Number(0)
            } else {
                db.set(newkey, val);
                Response::Number(1)
            }
        }
        "PING" => {
            let message = cmd.parse_args::<Option<Vec<u8>>>()?;
            Response::String(message.unwrap_or_else(|| b"PONG".to_vec()))
        }
        "SET" => {
            let (key, value) = cmd.parse_args::<(Vec<u8>, Vec<u8>)>()?;
            db.set(key, Value::String(value));
            Response::String(b"OK".to_vec())
        }
        "STRLEN" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            match db.get_str(&key)? {
                Some(s) => Response::Number(s.len() as _),
                None => Response::Number(0),
            }
        }
        "TYPE" => {
            let key = cmd.parse_args::<Vec<u8>>()?;
            let t: &[u8] = match db.get(&key) {
                Some(Value::String(_)) => b"string",
                Some(Value::List(_)) => b"list",
                Some(Value::Hash(_)) => b"hash",
                Some(Value::Set(_)) => b"set",
                None => b"none",
            };
            Response::String(t.to_vec())
        }
        "UNLINK" => {
            let keys = cmd.parse_args::<Vec<Vec<u8>>>()?;
            anyhow::ensure!(!keys.is_empty(), "expected UNLINK key [key ...]");
            Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _)
        }
        "COMMAND" => {
            // TODO: Implement this somehow
            Response::List(vec![])
        }
        _ => anyhow::bail!("Unrecognized command: {:?}", cmd.cmd()),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! exec_cmd {
        ($db:expr, $($cmd:expr),+) => {
            execute_command(&mut $db, Command::new(vec![$($cmd.as_bytes().to_vec(),)+]).unwrap()).unwrap()
        }
    }

    #[test]
    fn test_get_set() {
        let mut db = Database::new();
        assert_eq!(exec_cmd!(db, "GET", "a"), Response::Nil);

        assert_eq!(exec_cmd!(db, "SET", "a", "b"), Response::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(db, "GET", "a"), Response::String(b"b".to_vec()));

        assert_eq!(exec_cmd!(db, "SET", "a", "c"), Response::String(b"OK".to_vec()));
        assert_eq!(exec_cmd!(db, "GET", "a"), Response::String(b"c".to_vec()));

        assert_eq!(exec_cmd!(db, "DEL", "a"), Response::Number(1));
        assert_eq!(exec_cmd!(db, "DEL", "b"), Response::Number(0));
        assert_eq!(exec_cmd!(db, "DEL", "a", "b", "c"), Response::Number(0));
    }
}
