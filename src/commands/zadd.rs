use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::sorted_set::SortedSet;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zadd",
    arity: -4,
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
    let (key, members) = cmd.parse_args::<(ByteString, Vec<(i64, ByteString)>)>()?;
    let new = match db.get_zset(&key)? {
        Some(s) => {
            members.into_iter().map(|(score, v)| !s.insert(v, score).is_some()).filter(|&b| b).count()
        }
        None => {
            let mut s = SortedSet::new();
            let new = members.into_iter().map(|(score, v)| !s.insert(v, score).is_some()).filter(|&b| b).count();
            db.set(key, Value::ZSet(s));
            new
        }
    };
    Ok(Response::Number(new as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_zadd
        "zadd x 1 a 1 b" => 2;
        "zcard x"        => 2;
        "zadd x 2 a 3 c" => 1;
        "zcard x"        => 3;
    }
}
