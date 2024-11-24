use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"unlink",
    arity: -2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let keys = cmd.parse_args::<Vec<ByteString>>()?;
    anyhow::ensure!(!keys.is_empty(), "expected UNLINK key [key ...]");
    Ok(Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _))
}

#[cfg(test)]
crate::command_test! {
    "set x 0" => "OK";
    "set y 0" => "OK";
    "set z 0" => "OK";
    "exists x y z" => 3;
    "unlink x y"   => 2;
    "exists x y"   => 0;
    "unlink a b c" => 0;
    "unlink z a"   => 1;
    "exists z"     => 0;
}
