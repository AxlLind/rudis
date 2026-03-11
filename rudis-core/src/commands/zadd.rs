use ordered_float::NotNan;

use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zadd",
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
    let (key, members) = cmd.parse_args::<(ByteString, Vec<(f64, ByteString)>)>()?;
    let z = db.get_or_insert_zset(key)?;
    let mut added = 0;
    for (score, member) in members {
        let score = NotNan::new(score)?;
        if z.insert(score, member).is_none() {
            added += 1;
        }
    }
    Ok(Response::Number(added as _))
}

#[cfg(test)]
crate::command_test! {
    "zadd x 1 a 1 b" => 2;
    "zcard x"        => 2;
    "zadd x 2 a 3 c" => 1;
    "zcard x"        => 3;
}
