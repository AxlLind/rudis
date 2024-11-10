use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"sintercard",
    arity: -3,
    flags: &[
        b"readonly",
        b"movablekeys",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, keys) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
        let Some(mut set) = db.get_set(&key)?.cloned() else { return Ok(Response::Number(0)) };
        for k in keys {
            let Some(s) = db.get_set(&k)? else { continue };
            set.retain(|e| s.contains(e));
        }
        Ok(Response::Number(set.len() as _))
    }
}
