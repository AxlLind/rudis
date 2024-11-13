use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"hgetall",
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
    let pairs = match db.get_hash(&key)? {
        Some(h) => {
            let mut pairs = h.iter().collect::<Vec<_>>();
            pairs.sort();
            pairs.into_iter().flat_map(|(k, v)| [k, v]).cloned().collect()
        },
        None => Vec::new(),
    };
    Ok(Response::Array(pairs))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_hgetall
        "hset x a b x y" => 2;
        "hgetall x" => ["a", "b", "x", "y"];
        "hdel x a"  => 1;
        "hgetall x" => ["x", "y"];
        "hgetall q" => [];
    }
}
