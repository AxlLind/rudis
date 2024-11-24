use super::{int_from_bytes, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hincrby",
    arity: 4,
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
    let (key, field, increment) = cmd.parse_args::<(ByteString, ByteString, i64)>()?;
    let h = db.get_or_insert_hash(key)?;
    let n = h.get(&field).map_or(Ok(0), |v| int_from_bytes(v))? + increment;
    h.insert(field, n.to_string().into_bytes());
    Ok(Response::Number(n))
}

#[cfg(test)]
crate::command_test! {
    "hincrby x a 10" => 10;
    "hincrby x a -5" => 5;
    "hget x a"       => "5";
    "hincrby x a 20" => 25;
    "hincrby x y -3" => -3;
}
