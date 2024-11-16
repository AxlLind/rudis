use std::collections::HashSet;

use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
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

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
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

#[cfg(test)]
crate::command_test! {
    "sadd x 1 2 3"    => 3;
    "smove x y 1"     => 1;
    "smembers x"      => ["2", "3"];
    "smembers y"      => ["1"];
    "smove x y 2"     => 1;
    "smembers x"      => ["3"];
    "smembers y"      => ["1", "2"];
    "smove x y 1"     => 0;
    "smove q y 1"     => 0;
}
