use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

static INFO: CommandInfo = CommandInfo {
    name: b"set",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct SetCommand;

impl RedisCommand for SetCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, value) = cmd.parse_args::<(ByteString, ByteString)>()?;
        db.set(key, Value::String(value));
        Ok(Response::String(b"OK".to_vec()))
    }
}
