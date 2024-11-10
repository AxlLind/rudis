use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"pfdebug",
    arity: 3,
    flags: &[
        b"write",
        b"denyoom",
        b"admin",
    ],
    first_key: 2,
    last_key: 2,
    step: 1,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
