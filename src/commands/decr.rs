use super::{RedisCommand, incr_by};
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct DecrCommand;

impl RedisCommand for DecrCommand {
    fn name(&self) -> &'static str {
        "decr"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let key = cmd.parse_args::<ByteString>()?;
        incr_by(db, key, -1)
    }
}
