use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"flushdb",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct FlushdbCommand;

impl RedisCommand for FlushdbCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let arg = cmd.parse_args::<Option<ByteString>>()?;
        match arg.as_deref() {
            Some(b"SYNC") | None => {
                db.clear();
                Ok(Response::String(b"OK".to_vec()))
            },
            Some(b"ASYNC") => anyhow::bail!("async flush not implemented"),
            _ => anyhow::bail!("invalid argument"),
        }
    }
}
