use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"renamenx",
    arity: 3,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 2,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let n = if db.contains(&newkey) {
        0
    } else {
        let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
        db.set(newkey, val);
        1
    };
    Ok(Response::Number(n))
}

#[cfg(test)]
crate::command_test! {
    "set x 0"      => "OK";
    "renamenx x y" => 1;
    "exists x"     => 0;
    "get y"        => "0";
    "set x 1"      => "OK";
    "renamenx y x" => 0;
    "get x"        => "1";
    "renamenx x x" => 0;
}
