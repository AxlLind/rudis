use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"del",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};


pub struct DelCommand;

impl RedisCommand for DelCommand {
    fn name(&self) -> &'static [u8] { INFO.name }

    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let keys = cmd.parse_args::<Vec<ByteString>>()?;
        anyhow::ensure!(!keys.is_empty(), "expected DEL key [key ...]");
        Ok(Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _))
    }
}
