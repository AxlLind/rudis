use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"copy",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
    ],
    first_key: 1,
    last_key: 2,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (src, dst) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let value = match db.get(&src) {
        Some(v) => {
            let copy = v.clone();
            db.set(dst, copy);
            Response::Number(1)
        }
        None => Response::Number(0),
    };
    Ok(value)
}

#[cfg(test)]
crate::command_test! {
    "set x a"  => "OK";
    "copy x y" => 1;
    "get y"    => "a";
    "copy z y" => 0;
    "get y"    => "a";
}
