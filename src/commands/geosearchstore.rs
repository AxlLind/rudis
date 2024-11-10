use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"geosearchstore",
    arity: -8,
    flags: &[
        b"write",
        b"denyoom",
    ],
    first_key: 1,
    last_key: 2,
    step: 1,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
