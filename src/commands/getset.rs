use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"getset",
    arity: 3,
    flags: &[
        b"write",
        b"denyoom",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, value) = cmd.parse_args::<(ByteString, ByteString)>()?;
    Ok(match db.get_str(&key)? {
        Some(s) => Response::String(std::mem::replace(s, value)),
        None => Response::Nil,
    })
}
