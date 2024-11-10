use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"fcall_ro",
    arity: -3,
    flags: &[
        b"readonly",
        b"noscript",
        b"stale",
        b"skip_monitor",
        b"no_mandatory_keys",
        b"movablekeys",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
