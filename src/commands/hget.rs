use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hget",
    arity: 3,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, field) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let res = db.get_hash(&key)?
        .and_then(|h| h.get(&field))
        .map(|v| Response::String(v.clone()))
        .unwrap_or_default();
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "hset x a b x xyz" => 2;
    "hget x a"         => "b";
    "hget x x"         => "xyz";
    "hget x q"         => ();
    "hget q r"         => ();
}
