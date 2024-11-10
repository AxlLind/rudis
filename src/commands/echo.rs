use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"echo",
    arity: 2,
    flags: &[
        b"loading",
        b"stale",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, _: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let msg = cmd.parse_args::<ByteString>()?;
        Ok(Response::String(msg))
    }
}
