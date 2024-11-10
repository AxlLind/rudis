use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"append",
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
    let len = match db.get_str(&key)? {
        Some(v) => {
            v.extend(value);
            v.len()
        }
        None => {
            let len = value.len();
            db.set(key, Value::String(value));
            len
        }
    };
    Ok(Response::Number(len as _))
}
