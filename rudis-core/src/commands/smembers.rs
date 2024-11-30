use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"smembers",
    arity: 2,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let mut members = db.get_set(&key)?.map(|s| s.iter().cloned().collect::<Vec<_>>()).unwrap_or_default();
    members.sort();
    Ok(Response::string_array(members))
}

#[cfg(test)]
crate::command_test! {
    "sadd x 1 2 3" => 3;
    "smembers x"   => ["1", "2", "3"];
    "sadd x 3 4"   => 1;
    "smembers x"   => ["1", "2", "3", "4"];
    "smembers q"   => [];
}