use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"del",
    arity: -2,
    flags: &[
        b"write",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let keys = cmd.parse_args::<Vec<ByteString>>()?;
    anyhow::ensure!(!keys.is_empty(), "expected DEL key [key ...]");
    Ok(Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _))
}

#[cfg(test)]
crate::command_test! {
    "del x y z"    => 0;
    "set x 0"      => "OK";
    "del x"        => 1;
    "set x 1"      => "OK";
    "set y 2"      => "OK";
    "set z 3"      => "OK";
    "del x y z"    => 3;
    "exists x y z" => 0;
}
