use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"rename",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
        let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
        db.set(newkey, val);
        Ok(Response::String(b"OK".to_vec()))
    }
}
