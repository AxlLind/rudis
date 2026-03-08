use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"mset",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
    ],
    first_key: 1,
    last_key: -1,
    step: 2,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let ops = cmd.parse_args::<Vec<(ByteString, ByteString)>>()?;
    for (key, value) in ops {
        db.set(key, Value::String(value));
    }
    Ok(Response::SimpleString(b"OK".to_vec()))
}

#[cfg(test)]
crate::command_test! {
    "mset a 1 b 2" => "OK";
    "mget a b" => ["1", "2"];
}
