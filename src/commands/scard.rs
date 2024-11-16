use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"scard",
    arity: 2,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let card = db.get_set(&key)?.map(|s| s.len()).unwrap_or(0);
    Ok(Response::Number(card as _))
}

#[cfg(test)]
crate::command_test! {
    "scard x"      => 0;
    "sadd x 1 2 3" => 3;
    "scard x"      => 3;
    "sadd x 4"     => 1;
    "scard x"      => 4;
}
