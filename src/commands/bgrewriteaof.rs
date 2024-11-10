use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"bgrewriteaof",
    arity: 1,
    flags: &[
        b"admin",
        b"noscript",
        b"no_async_loading",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, _: Command) -> anyhow::Result<Response> {
    anyhow::bail!("unimplemented")
}
