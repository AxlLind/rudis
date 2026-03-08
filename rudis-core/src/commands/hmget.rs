use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hmget",
    arity: -3,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, fields) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!fields.is_empty(), "expected HMGET key field [field..]");
    let hash = db.get_hash(&key)?;
    let res = fields.iter().map(|f| {
        hash.as_ref()
            .map(|h| h.get(f))
            .flatten()
            .map(|v| Response::BulkString(v.clone()))
            .unwrap_or_default()
    }).collect();
    Ok(Response::Array(res))
}

#[cfg(test)]
crate::command_test! {
    "hset x a b x y" => 2;
    "hmget x a"      => ["b"];
    "hmget x a x"    => ["b", "y"];
}
