use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

pub struct SetCommand;

impl RedisCommand for SetCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, value) = cmd.parse_args::<(ByteString, ByteString)>()?;
        db.set(key, Value::String(value));
        Ok(Response::String(b"OK".to_vec()))
    }
}
