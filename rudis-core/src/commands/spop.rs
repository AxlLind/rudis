use std::collections::HashSet;

use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"spop",
    arity: -2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

fn set_pop(set: &mut HashSet::<ByteString>) -> Option<ByteString> {
    let e = set.iter().next()?.clone();
    set.remove(&e);
    Some(e)
}

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, maybe_count) = cmd.parse_args::<(ByteString, Option<i64>)>()?;
    let Some(set) = db.get_set(&key)? else { return Ok(Response::Nil) };
    let res = match maybe_count {
        Some(count) => {
            let elems = (0..count).filter_map(|_| set_pop(set).map(|e| Response::BulkString(e))).collect();
            Response::Array(elems)
        }
        None => {
            set_pop(set).map(|e| Response::BulkString(e)).unwrap_or_default()
        }
    };
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "sadd x 1 2 3" => 3;
    "spop x 3"     => ["1", "2", "3"] ignore_order;
    "scard x"      => 0;
    "sadd x 1"     => 1;
    "spop x"       => "1";
}
