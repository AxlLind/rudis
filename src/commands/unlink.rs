use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"unlink",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct UnlinkCommand;

impl RedisCommand for UnlinkCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let keys = cmd.parse_args::<Vec<ByteString>>()?;
        anyhow::ensure!(!keys.is_empty(), "expected UNLINK key [key ...]");
        Ok(Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _))
    }
}
