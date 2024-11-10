use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sintercard",
    arity: -3,
    flags: &[
        b"readonly",
        b"movablekeys",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, keys) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    let Some(mut set) = db.get_set(&key)?.cloned() else { return Ok(Response::Number(0)) };
    for k in keys {
        let Some(s) = db.get_set(&k)? else { continue };
        set.retain(|e| s.contains(e));
    }
    Ok(Response::Number(set.len() as _))
}
