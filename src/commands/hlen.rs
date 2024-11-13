use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hlen",
    arity: 2,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let key = cmd.parse_args::<ByteString>()?;
    let len = db.get_hash(&key)?.map(|h| h.len()).unwrap_or(0);
    Ok(Response::Number(len as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_hlen
        "hset x a b x y" => 2;
        "hlen x"         => 2;
        "hset x a c z 1" => 2;
        "hlen x"         => 3;
        "hdel x a"       => 1;
        "hlen x"         => 2;
        "hlen q"         => 0;
    }
}
