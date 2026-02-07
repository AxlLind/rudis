use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"mget",
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
    let mut values = Vec::new();
    for k in keys {
        let res = db.get_str(&k)?.map(|s| Response::BulkString(s.clone())).unwrap_or_default();
        values.push(res);
    }
    Ok(Response::Array(values))
}

#[cfg(test)]
crate::command_test! {
    "set a 123" => "OK";
    "set b wow" => "OK";
    "mget a b" => ["123", "wow"];
}
