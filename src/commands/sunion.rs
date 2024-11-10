use std::collections::HashSet;

use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sunion",
    arity: -2,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let keys = cmd.parse_args::<Vec<ByteString>>()?;
    anyhow::ensure!(!keys.is_empty(), "expected SUNION key [key ...]");
    let mut set = HashSet::new();
    for k in keys {
        let Some(s) = db.get_set(&k)? else { continue };
        for m in s.iter() {
            if !set.contains(m) {
                set.insert(m.clone());
            }
        }
    }
    let mut elems = set.into_iter().collect::<Vec<_>>();
    elems.sort();
    Ok(Response::Array(elems))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_sunion
        "sadd x 1 2 3" => 3;
        "sadd y 2 3 4" => 3;
        "sadd z 3 4 5" => 3;
        "sunion x"     => ["1", "2", "3"];
        "sunion x z"   => ["1", "2", "3", "4", "5"];
        "sunion x y z" => ["1", "2", "3", "4", "5"];
        "sunion x y"   => ["1", "2", "3", "4"];
        "sunion y z"   => ["2", "3", "4", "5"];
        "sunion q"     => [];
        "sunion q x"   => ["1", "2", "3"];
    }
}
