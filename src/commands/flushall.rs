use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"flushall",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct FlushallCommand;

impl RedisCommand for FlushallCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let _mode = cmd.parse_args::<Option<ByteString>>()?;
        db.clear();
        Ok(Response::String(b"OK".to_vec()))
    }
}
