use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sadd",
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
    let (key, elems) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!elems.is_empty(), "expected SADD member [member ...]");
    let s = db.get_or_insert_set(key)?;
    let prelen = s.len();
    s.extend(elems);
    Ok(Response::Number((s.len() - prelen) as _))
}

#[cfg(test)]
crate::command_test! {
    "sadd x 1 2 3" => 3;
    "sadd x 1 2 3" => 0;
    "sadd x 3 4" => 1;
}
