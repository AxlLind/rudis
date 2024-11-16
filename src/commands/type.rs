use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"type",
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
    let t: &[u8] = match db.get(&key) {
        Some(Value::String(_)) => b"string",
        Some(Value::Array(_)) => b"list",
        Some(Value::Hash(_)) => b"hash",
        Some(Value::Set(_)) => b"set",
        Some(Value::ZSet(_)) => b"zset",
        None => b"none",
    };
    Ok(Response::String(t.to_vec()))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_type
        "type x"     => "none";
        "set x 0"    => "OK";
        "type x"     => "string";
        "rpush y 1"  => 1;
        "type y"     => "list";
        "sadd s 1"   => 1;
        "type s"     => "set";
        "hset h x y" => 1;
        "type h"     => "hash";
        "zadd z 1 a" => 1;
        "type z"     => "zset";
    }
}
