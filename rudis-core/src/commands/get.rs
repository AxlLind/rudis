use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"get",
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
    let res = db.get_str(&key)?.map(|s| Response::SimpleString(s.clone())).unwrap_or_default();
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "set mykey 10" => "OK";
    "get mykey"    => "10";
    "get x"        => ();
}
