use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"del",
    arity: -2,
    flags: &[
        b"write",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let keys = cmd.parse_args::<Vec<ByteString>>()?;
        anyhow::ensure!(!keys.is_empty(), "expected DEL key [key ...]");
        Ok(Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _))
    }
}
