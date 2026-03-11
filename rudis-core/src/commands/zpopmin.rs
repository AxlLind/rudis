use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zpopmin",
    arity: -2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, maybe_count) = cmd.parse_args::<(ByteString, Option<i64>)>()?;
    let Some(set) = db.get_zset(&key)? else { return Ok(Response::Array(Vec::new())) };
    let res = (0..maybe_count.unwrap_or(1))
        .filter_map(|_| set.popmin())
        .flat_map(|(s, m)| [Response::BulkString(m), Response::float(s)])
        .collect();
    Ok(Response::Array(res))
}

#[cfg(test)]
crate::command_test! {
    "zadd x 1 a 2 b 3 c 4 d" => 4;
    "zpopmin x 3" => ["a", "1", "b", "2", "c", "3"];
    "zcard x" => 1;
    "zpopmin x" => ["d", "4"];
}
