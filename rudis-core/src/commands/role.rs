use super::CommandInfo;
use crate::command::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"role",
    arity: 1,
    flags: &[
        b"noscript",
        b"loading",
        b"stale",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, cmd: Command) -> anyhow::Result<Response> {
    anyhow::ensure!(!cmd.has_more(), "wrong number of arguments");
    let res = Response::Array(vec![
        Response::BulkString(b"master".to_vec()),
        Response::Number(0),
        Response::Array(Vec::new()),
    ]);
    Ok(res)
}

// TODO: cannot test, framework does not support nested arrays
// #[cfg(test)]
// crate::command_test! {
//     "role" => ["master", 0, []];
// }
