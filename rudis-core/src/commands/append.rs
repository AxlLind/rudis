use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

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
    let v = db.get_or_insert_str(key)?;
    v.extend(value);
    Ok(Response::Number(v.len() as _))
}

#[cfg(test)]
crate::command_test! {
    "append x abc" => 3;
    "append x def" => 6;
    "get x"        => "abcdef";
    "set y 123"    => "OK";
    "append y 4"   => 4;
    "get y"        => "1234";
}
