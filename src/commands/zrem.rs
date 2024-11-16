use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zrem",
    arity: -3,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, members) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!members.is_empty(), "expected ZREM key member [member..]");
    let removed = db.get_zset(&key)?
        .map(|s| members.iter().filter_map(|m| s.remove(m)).count())
        .unwrap_or(0);
    Ok(Response::Number(removed as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_zrem
        "zadd x 1 a 1 b" => 2;
        "zrem x b c"     => 1;
        "zcard x"        => 1;
        "zrem x x y z"   => 0;
        "zrem x a"       => 1;
        "zrem q a b"     => 0;
    }
}
