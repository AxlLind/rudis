use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sunsubscribe",
    arity: -1,
    flags: &[
        b"pubsub",
        b"noscript",
        b"loading",
        b"stale",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
