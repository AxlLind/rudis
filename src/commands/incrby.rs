use super::{incr_by, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct IncrbyCommand;

impl RedisCommand for IncrbyCommand {
    fn name(&self) -> &'static str {
        "incrby"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, step) = cmd.parse_args::<(ByteString, i64)>()?;
        incr_by(db, key, step)
    }
}
