use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

static INFO: CommandInfo = CommandInfo {
    name: b"type",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct TypeCommand;

impl RedisCommand for TypeCommand {
    fn name(&self) -> &'static [u8] { INFO.name }

    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let key = cmd.parse_args::<ByteString>()?;
        let t: &[u8] = match db.get(&key) {
            Some(Value::String(_)) => b"string",
            Some(Value::Array(_)) => b"list",
            Some(Value::Hash(_)) => b"hash",
            Some(Value::Set(_)) => b"set",
            None => b"none",
        };
        Ok(Response::String(t.to_vec()))
    }
}
