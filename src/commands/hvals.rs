use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hvals",
    arity: 2,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let mut keys = db.get_hash(&key)?
        .map(|h| h.values().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    keys.sort();
    Ok(Response::Array(keys))
}

#[cfg(test)]
crate::command_test! {
    "hset x a b x y" => 2;
    "hvals x"        => ["b", "y"];
    "hdel x a"       => 1;
    "hvals x"        => ["y"];
    "hvals q"        => [];
}
