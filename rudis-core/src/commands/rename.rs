use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"rename",
    arity: 3,
    flags: &[
        b"write",
    ],
    first_key: 1,
    last_key: 2,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
    db.set(newkey, val);
    Ok(Response::SimpleString(b"OK".to_vec()))
}

#[cfg(test)]
crate::command_test! {
    "set x 0"    => "OK";
    "rename x y" => "OK";
    "exists x"   => 0;
    "get y"      => "0";
    "set x 1"    => "OK";
    "rename y x" => "OK";
    "get x"      => "0";
    "rename x x" => "OK";
    "get x"      => "0";
}
