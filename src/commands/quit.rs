use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"quit",
    arity: -1,
    flags: &[
        b"noscript",
        b"loading",
        b"stale",
        b"fast",
        b"no_auth",
        b"allow_busy",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, _: &mut Database, _: Command) -> anyhow::Result<Response> {
        unreachable!()
    }
}
