use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"lpop",
    arity: -2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, count) = cmd.parse_args::<(ByteString, Option<i64>)>()?;
    let Some(list) = db.get_list(&key)? else { return Ok(Response::Nil) };
    Ok(match count {
        Some(n) if n < 0 => anyhow::bail!("value is out of range, must be positive"),
        Some(n) => {
            let n = list.len().min(n as _);
            let mut x = list.split_off(n);
            std::mem::swap(&mut x, list);
            Response::Array(x)
        }
        None => if list.is_empty() { Response::Nil } else { Response::String(list.remove(0)) },
    })
}

#[cfg(test)]
crate::command_test! {
    "lpop x"            => ();
    "rpush x 1 2 3 4 5" => 5;
    "lpop x"            => "1";
    "lrange x 0 -1"     => ["2", "3", "4", "5"];
    "lpop x 3"          => ["2", "3", "4"];
    "lrange x 0 -1"     => ["5"];
}
