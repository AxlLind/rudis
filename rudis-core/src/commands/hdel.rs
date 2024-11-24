use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hdel",
    arity: -3,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, fields) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!fields.is_empty(), "expected HDEL key field [field ..]");
    let deleted = db.get_hash(&key)?
        .map(|h| fields.iter().filter_map(|f| h.remove(f)).count())
        .unwrap_or(0);
    Ok(Response::Number(deleted as _))
}

#[cfg(test)]
crate::command_test! {
    "hset x a b x y" => 2;
    "hdel x a b"     => 1;
    "hlen x"         => 1;
    "hdel x x"       => 1;
    "hlen x"         => 0;
    "hdel q a b c"   => 0;
}
