use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct FlushallCommand;

impl RedisCommand for FlushallCommand {
    fn name(&self) -> &'static str {
        "flushall"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let _mode = cmd.parse_args::<Option<ByteString>>()?;
        db.clear();
        Ok(Response::String(b"OK".to_vec()))
    }
}
