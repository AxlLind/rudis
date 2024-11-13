use std::collections::HashMap;

use super::{int_from_bytes, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hincrby",
    arity: 4,
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
    let (key, field, increment) = cmd.parse_args::<(ByteString, ByteString, i64)>()?;
    let v = match db.get_hash(&key)? {
        Some(h) => {
            let n = h.get(&field).map_or(Ok(0), |v| int_from_bytes(v))? + increment;
            h.insert(field, n.to_string().into_bytes());
            n
        }
        None => {
            let mut h = HashMap::new();
            h.insert(field, increment.to_string().into_bytes());
            db.set(key, Value::Hash(h));
            increment
        }
    };
    Ok(Response::Number(v))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_hincrby
        "hincrby x a 10" => 10;
        "hincrby x a -5" => 5;
        "hget x a"       => "5";
        "hincrby x a 20" => 25;
        "hincrby x y -3" => -3;
    }
}
