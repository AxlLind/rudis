use super::{clamp_range, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"lrange",
    arity: 4,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, start, stop) = cmd.parse_args::<(ByteString, i64, i64)>()?;
    let range = db.get_array(&key)?.map(|a| {
        let (start, stop) = clamp_range(a.len(), start, stop);
        a[start..=stop].to_vec()
    }).unwrap_or_default();
    Ok(Response::Array(range))
}

#[cfg(test)]
crate::command_test! {
    "lrange x 0 -1"   => [];
    "rpush x 1 2 3 4" => 4;
    "lrange x 0 0"    => ["1"];
    "lrange x 0 10"   => ["1", "2", "3", "4"];
    "lrange x 0 -1"   => ["1", "2", "3", "4"];
    "lrange x 0 -2"   => ["1", "2", "3"];
}
