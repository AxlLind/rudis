use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hstrlen",
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
    let len = db.get_hash(&key)?.and_then(|h| h.get(&field)).map(|s| s.len()).unwrap_or(0);
    Ok(Response::Number(len as _))
}

#[cfg(test)]
crate::command_test! {
    "hset x a b y abc"   => 2;
    "hstrlen x a"        => 1;
    "hstrlen x y"        => 3;
    "hstrlen x q"        => 0;
    "hstrlen q r"        => 0;
}
