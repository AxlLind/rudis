use super::{RedisCommand, incr_by};
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct DecrbyCommand;

impl RedisCommand for DecrbyCommand {
    fn name(&self) -> &'static str {
        "decrby"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, step) = cmd.parse_args::<(ByteString, i64)>()?;
        incr_by(db, key, -step)
    }
}
