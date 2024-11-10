use std::collections::HashSet;

use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

static INFO: CommandInfo = CommandInfo {
    name: b"smove",
    arity: 4,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 2,
    step: 1,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (src, dst, member) = cmd.parse_args::<(ByteString, ByteString, ByteString)>()?;

        let Some(src_set) = db.get_set(&src)? else { return Ok(Response::Number(0)) };
        if !src_set.remove(&member) {
            return Ok(Response::Number(0));
        }

        match db.get_set(&dst)? {
            Some(dst_set) => { dst_set.insert(member); }
            None => { db.set(dst, Value::Set(HashSet::from([member]))); }
        }
        Ok(Response::Number(1))
    }
}
