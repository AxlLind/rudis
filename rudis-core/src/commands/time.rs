use std::time::{SystemTime, UNIX_EPOCH};
use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"time",
    arity: 1,
    flags: &[
        b"loading",
        b"stale",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, cmd: Command) -> anyhow::Result<Response> {
    anyhow::ensure!(!cmd.has_more(), "unexpected arguments");
    let t = SystemTime::now().duration_since(UNIX_EPOCH).expect("now is later than unix epoch");
    let s = t.as_secs().to_string().into_bytes();
    let ms = t.subsec_micros().to_string().into_bytes();
    Ok(Response::string_array(vec![s, ms]))
}

// TODO: how to test this?
// #[cfg(test)]
// crate::command_test! {
//     "time" => ["1", "0"];
// }
