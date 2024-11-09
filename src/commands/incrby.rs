use super::{incr_by, CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"incrby",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct IncrbyCommand;

impl RedisCommand for IncrbyCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, step) = cmd.parse_args::<(ByteString, i64)>()?;
        incr_by(db, key, step)
    }
}
