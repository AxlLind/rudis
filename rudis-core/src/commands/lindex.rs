use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"lindex",
    arity: 3,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, index) = cmd.parse_args::<(ByteString, i64)>()?;
    let res = db.get_array(&key)?.map(|list| {
        let len = list.len() as i64;
        let i = if index < 0 {len + index} else {index};
        if i < 0 || i >= len {
            return Response::Nil;
        }
        Response::SimpleString(list[i as usize].clone())
    }).unwrap_or_default();
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "lindex l 0" => ();
    "lindex l -1" => ();
    "lindex l 99999" => ();
    "rpush l 1 2 3 4" => 4;
    "lindex l 0" => "1";
    "lindex l 3" => "4";
    "lindex l 4" => ();
    "lindex l 99999" => ();
    "lindex l -1" => "4";
    "lindex l -2" => "3";
    "lindex l -4" => "1";
    "lindex l -5" => ();
    "lindex l -99999" => ();
}
