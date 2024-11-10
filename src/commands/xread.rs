use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"xread",
    arity: -4,
    flags: &[
        b"readonly",
        b"blocking",
        b"movablekeys",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
