use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"getdel",
    arity: 2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let res = db.get_str(&key)?.cloned().map(|s| {
        db.del(&key);
        Response::String(s)
    }).unwrap_or_default();
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "getdel x" => ();
    "set x 0"  => "OK";
    "getdel x" => "0";
    "exists x" => 0;
}
