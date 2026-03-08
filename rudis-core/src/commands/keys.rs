use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"keys",
    arity: 2,
    flags: &[
        b"readonly",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let pattern = cmd.parse_args::<ByteString>()?;
    let patten_str = str::from_utf8(&pattern)?;
    let keys = db.keys().filter_map(|k| {
        let k_str = str::from_utf8(k).ok()?;
        glob_match::glob_match(patten_str, k_str).then(|| Response::BulkString(k.to_vec()))
    }).collect::<Vec<_>>();
    Ok(Response::Array(keys))
}

#[cfg(test)]
crate::command_test! {
    "MSET a1 1 b1 1" => "OK";
    "KEYS *a1*"      => ["a1"];
    "KEYS *b*"       => ["b1"];
    "KEYS [^a]1"     => ["b1"];
    // TODO: cant test this due to unpredictable order
    // "KEYS [ab]1"     => ["a1", "b1"];
    // "KEYS *1"        => ["a1", "b1"];
    // "KEYS ?1"        => ["a1", "b1"];
    "KEYS abc"       => [];
}
