use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"substr",
    arity: 4,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, cmd: Command) -> anyhow::Result<Response> {
    // deprecated alias for getrange
    crate::commands::getrange::run(db, cmd)
}

#[cfg(test)]
crate::command_test! {
    "set x this_is_a_string" => "OK";
    "substr x 0 3"           => "this";
    "substr x -3 -1"         => "ing";
    "substr x 0 -1"          => "this_is_a_string";
    "substr x 10 100"        => "string";
    "substr q 0 -1"          => "";
}
