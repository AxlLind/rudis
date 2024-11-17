use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hset",
    arity: -4,
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
    let (key, fields) = cmd.parse_args::<(ByteString, Vec<(ByteString, ByteString)>)>()?;
    anyhow::ensure!(!fields.is_empty(), "expected HSET key field value [field value ..]");
    let len = fields.len();
    let h = db.get_or_insert_hash(key)?;
    for (k, v) in fields {
        h.insert(k, v);
    }
    Ok(Response::Number(len as _))
}

#[cfg(test)]
crate::command_test! {
    "hset x a b x y" => 2;
    "hget x a" => "b";
    "hget x x" => "y";
}
