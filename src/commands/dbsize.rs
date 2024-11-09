use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"dbsize",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct DbsizeCommand;

impl RedisCommand for DbsizeCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, cmd: Command) -> anyhow::Result<Response> {
        anyhow::ensure!(!cmd.has_more(), "got extra arguments");
        Ok(Response::Number(db.state.len() as _))
    }
}
