use super::{clamp_range, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"getrange",
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
    let range = db.get_str(&key)?.map(|s| {
        let (start, stop) = clamp_range(s.len(), start, stop);
        s[start..=stop].to_vec()
    }).unwrap_or_default();
    Ok(Response::String(range))
}

#[cfg(test)]
crate::command_test! {
    "set x this_is_a_string" => "OK";
    "getrange x 0 3"         => "this";
    "getrange x -3 -1"       => "ing";
    "getrange x 0 -1"        => "this_is_a_string";
    "getrange x 10 100"      => "string";
    "getrange q 0 -1"        => "";
}
