use ordered_float::NotNan;

use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zincrby",
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
    let (key, increment, member) = cmd.parse_args::<(ByteString, f64, ByteString)>()?;
    let zset = db.get_or_insert_zset(key)?;
    let old_score = zset.remove(member.clone()).map(|s| *s).unwrap_or(0.0);
    let score = old_score + increment;
    zset.insert(NotNan::new(score)?, member);
    Ok(Response::float(score))
}

#[cfg(test)]
crate::command_test! {
    "zadd z 1 a"       => 1;
    "zincrby z -0.1 a" => "0.9";
    "zscore z a"       => "0.9";
    "zincrby z 14 b"   => "14";
}
