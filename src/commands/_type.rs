use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

pub struct TypeCommand;

impl RedisCommand for TypeCommand {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let key = cmd.parse_args::<ByteString>()?;
        let t: &[u8] = match db.get(&key) {
            Some(Value::String(_)) => b"string",
            Some(Value::List(_)) => b"list",
            Some(Value::Hash(_)) => b"hash",
            Some(Value::Set(_)) => b"set",
            None => b"none",
        };
        Ok(Response::String(t.to_vec()))
    }
}
