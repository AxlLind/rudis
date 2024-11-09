use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"renamenx",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct RenamenxCommand;

impl RedisCommand for RenamenxCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
        let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
        let n = if db.is_set(&newkey) {
            0
        } else {
            db.set(newkey, val);
            1
        };
        Ok(Response::Number(n))
    }
}
