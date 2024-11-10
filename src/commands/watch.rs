use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"watch",
    arity: -2,
    flags: &[
        b"noscript",
        b"loading",
        b"stale",
        b"fast",
        b"allow_busy",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
