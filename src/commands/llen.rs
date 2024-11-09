use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"llen",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct LlenCommand;

impl RedisCommand for LlenCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let key = cmd.parse_args::<ByteString>()?;
        let len = db.get_list(&key)?.map(|l| l.len()).unwrap_or(0);
        Ok(Response::Number(len as _))
    }
}
