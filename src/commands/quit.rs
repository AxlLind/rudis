use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"quit",
    arity: -1,
    flags: &[
        b"noscript",
        b"loading",
        b"stale",
        b"fast",
        b"no_auth",
        b"allow_busy",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, cmd: Command) -> anyhow::Result<Response> {
    anyhow::ensure!(!cmd.has_more(), "expected no arguments for quit");
    Ok(Response::String(b"OK".to_vec()))
}

#[cfg(test)]
crate::command_test! {
    "quit" => "OK";
}
