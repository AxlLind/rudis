use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hexists",
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
    let (key, field) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let exists = db.get_hash(&key)?.map(|h| h.contains_key(&field)).unwrap_or(false);
    Ok(Response::Number(exists as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_hexists
        "hset x a b x y" => 2;
        "hexists x a" => 1;
        "hexists x x" => 1;
        "hexists x b" => 0;
        "hexists q r" => 0;
    }
}
