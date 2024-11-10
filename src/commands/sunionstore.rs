use std::collections::HashSet;

use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sunionstore",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (dest, keys) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!keys.is_empty(), "expected SUNION dest key [key ...]");
    let mut set = HashSet::new();
    for k in &keys {
        let Some(s) = db.get_set(k)? else { continue };
        for m in s.iter() {
            if !set.contains(m) {
                set.insert(m.clone());
            }
        }
    }
    let len = set.len();
    db.set(dest, Value::Set(set));
    Ok(Response::Number(len as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_sunionstore
        "sadd x 1 2 3"        => 3;
        "sadd y 2 3 4"        => 3;
        "sadd z 3 4 5"        => 3;
        "sunionstore r x"     => 3;
        "smembers r"          => ["1", "2", "3"];
        "sunionstore r x z"   => 5;
        "smembers r"          => ["1", "2", "3", "4", "5"];
        "sunionstore r x y z" => 5;
        "smembers r"          => ["1", "2", "3", "4", "5"];
        "sunionstore r x y"   => 4;
        "smembers r"          => ["1", "2", "3", "4"];
        "sunionstore r y z"   => 4;
        "smembers r"          => ["2", "3", "4", "5"];
        "sunionstore r q"     => 0;
        "smembers r"          => [];
        "sunionstore r q x"   => 3;
        "smembers r"          => ["1", "2", "3"];
    }
}
