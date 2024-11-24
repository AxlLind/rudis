use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"exists",
    arity: -2,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let keys = cmd.parse_args::<Vec<ByteString>>()?;
    anyhow::ensure!(!keys.is_empty(), "expected EXISTS key [key ...]");
    Ok(Response::Number(keys.iter().filter(|&key| db.contains(key)).count() as _))
}

#[cfg(test)]
crate::command_test! {
    "exists x" => 0;
    "exists a b c " => 0;
    "set x 1" => "OK";
    "exists x" => 1;
    "exists z y x" => 1;
    "sadd y a" => 1;
    "exists x y z" => 2;
}
