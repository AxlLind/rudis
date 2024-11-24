use super::{incr_by, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"decr",
    arity: 2,
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
    let key = cmd.parse_args::<ByteString>()?;
    incr_by(db, key, -1)
}

#[cfg(test)]
crate::command_test! {
    "decr x"     => -1;
    "decr x"     => -2;
    "set x 1234" => "OK";
    "decr x"     => 1233;
}
