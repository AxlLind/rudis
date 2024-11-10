use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sismember",
    arity: 3,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, member) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let is_member = db.get_set(&key)?.map(|s| s.contains(&member)).unwrap_or(false);
    Ok(Response::Number(is_member as _))
}
