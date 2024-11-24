use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zscore",
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
    let (key, member) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let res = db.get_zset(&key)?
        .and_then(|z| z.get_score(&member))
        .map(Response::Number)
        .unwrap_or_default();
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "zadd x 1 a 99 b" => 2;
    "zscore x a"      => 1;
    "zscore x b"      => 99;
    "zscore x c"      => ();
    "zscore q a"      => ();
}
