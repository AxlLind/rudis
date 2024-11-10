use anyhow::Ok;

use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"sdiff",
    arity: -2,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, keys) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
        let Some(mut set) = db.get_set(&key)?.cloned() else { return Ok(Response::Array(Vec::new())) };
        for k in keys {
            let Some(s) = db.get_set(&k)? else { continue };
            set.retain(|e| !s.contains(e));
        }
        let mut elems = set.into_iter().collect::<Vec<_>>();
        elems.sort();
        Ok(Response::Array(elems))
    }
}
