use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zcount",
    arity: 4,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, min, max) = cmd.parse_args::<(ByteString, i64, i64)>()?;
    let count = db.get_zset(&key)?.map(|z| z.range(min, max).count()).unwrap_or(0);
    Ok(Response::Number(count as _))
}

#[cfg(test)]
crate::command_test! {
    "zadd x 1 a 2 b 3 c 3 d 4 e" => 5;
    "zcount x -10 1"             => 1;
    "zcount x 1 4"               => 5;
    "zcount x -10 1000"          => 5;
    "zcount x 2 3"               => 3;
    "zcount q 0 1"               => 0;
}
