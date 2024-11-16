use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

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
        Some(s) => {
            let prev = std::mem::replace(s, value);
            Response::String(prev)
        },
        None => {
            db.set(key, Value::String(value));
            Response::Nil
        },
    })
}

#[cfg(test)]
crate::command_test! {
    "getset x 0" => ();
    "get x"      => "0";
    "getset x 1" => "0";
    "get x"      => "1";
}
