use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"lpush",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, elements) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!elements.is_empty(), "expected LPUSH key element [element ...]");
    let a = db.get_or_insert_array(key)?;
    for e in elements {
        a.insert(0, e);
    }
    Ok(Response::Number(a.len() as _))
}

#[cfg(test)]
crate::command_test! {
    "lpush x 1 2 3"   => 3;
    "lrange x 0 -1"   => ["3", "2", "1"];
    "lpush x 4 5"     => 5;
    "lrange x 0 -1"   => ["5", "4", "3", "2", "1"];
    "lpush x 6"       => 6;
    "lrange x 0 -1"   => ["6", "5", "4", "3", "2", "1"];
}
