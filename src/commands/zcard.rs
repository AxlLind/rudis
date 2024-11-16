use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zcard",
    arity: 2,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let len = db.get_zset(&key)?.map(|s| s.len()).unwrap_or(0);
    Ok(Response::Number(len as _))
}

#[cfg(test)]
crate::command_test! {
    "zcard x"        => 0;
    "zadd x 1 a 1 b" => 2;
    "zcard x"        => 2;
    "zadd x 2 a 3 c" => 1;
    "zcard x"        => 3;
}
