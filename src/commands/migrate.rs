use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"migrate",
    arity: -6,
    flags: &[
        b"write",
        b"movablekeys",
    ],
    first_key: 3,
    last_key: 3,
    step: 1,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
