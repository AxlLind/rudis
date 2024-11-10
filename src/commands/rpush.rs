use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Value, ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"rpush",
    arity: -3,
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
    let (key, elements) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!elements.is_empty(), "expected RPUSH key element [element ...]");
    Ok(match db.get_list(&key)? {
        Some(list) => {
            list.extend(elements);
            Response::Number(list.len() as _)
        }
        None => {
            let len = elements.len();
            db.set(key, Value::Array(elements));
            Response::Number(len as _)
        }
    })
}
