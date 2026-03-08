use super::CommandInfo;
use crate::command::Command;
use crate::commands::parse_from_bytes;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"incrbyfloat",
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
    let (key, step) = cmd.parse_args::<(ByteString, f64)>()?;
    let val = step + match db.get_str(&key)? {
        Some(v) => parse_from_bytes(v)?,
        None => 0.0,
    };
    let val = val.to_string().into_bytes();
    db.set(key, Value::String(val.clone()));
    Ok(Response::BulkString(val))
}

#[cfg(test)]
crate::command_test! {
    "incrbyfloat x 10.7" => 10.7;
    "incrbyfloat x -1"   => 9.7;
    "set x 234.12"       => "OK";
    "incrbyfloat x 1000" => 1234.12;
    "get x"              => "1234.12";
}
