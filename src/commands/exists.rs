use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"exists",
    arity: -2,
    flags: &[
        b"readonly",
        b"fast",
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
        anyhow::ensure!(!keys.is_empty(), "expected EXISTS key [key ...]");
        Ok(Response::Number(keys.iter().filter(|&key| db.contains(key)).count() as _))
    }
}
