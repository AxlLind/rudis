use super::{incr_by, CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"incrby",
    arity: 3,
    flags: &[
        b"write",
        b"denyoom",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, step) = cmd.parse_args::<(ByteString, i64)>()?;
        incr_by(db, key, step)
    }
}
