use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"dbsize",
    arity: 1,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(db: &mut Database, cmd: Command) -> anyhow::Result<Response> {
    anyhow::ensure!(!cmd.has_more(), "got extra arguments");
    Ok(Response::Number(db.state.len() as _))
}
