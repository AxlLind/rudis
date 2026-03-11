use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"zrank",
    arity: 3,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, member) = cmd.parse_partial_args::<(ByteString, ByteString)>()?;
    let withscore = cmd.parse_option("WITHSCORE");
    cmd.ensure_empty()?;

    let Some(set) = db.get_zset(&key)? else { return Ok(Response::Nil) };
    let Some((score, rank)) = set.rank(member) else { return Ok(Response::Nil) };
    let res = if withscore {
        Response::Array(vec![Response::Number(rank as _), Response::Number(score)])
    } else {
        Response::Number(rank as _)
    };
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "zadd z b 1 a 1 c 2 d 3" => 4;
    "zrank z x" => ();
    "zrank z a" => 0;
    "zrank z b" => 1;
    "zrank z c" => 2;
    "zrank z d" => 3;
    // TODO: how to test do test with array of different element types?
    // "zrank z a WITHSCORE" => [0, "1"];
}
