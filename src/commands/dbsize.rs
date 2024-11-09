use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"dbsize",
    arity: 1,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, cmd: Command) -> anyhow::Result<Response> {
        anyhow::ensure!(!cmd.has_more(), "got extra arguments");
        Ok(Response::Number(db.state.len() as _))
    }
}
