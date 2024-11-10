use super::{clamp_range, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"lrange",
    arity: 4,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, start, stop) = cmd.parse_args::<(ByteString, i64, i64)>()?;
    Ok(match db.get_list(&key)? {
        Some(list) => {
            let (start, stop) = clamp_range(list.len(), start, stop);
            Response::Array(list[start..=stop].to_vec())
        }
        None => Response::Array(Vec::new()),
    })
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_lrange
        "lrange x 0 -1"   => [];
        "rpush x 1 2 3 4" => 4;
        "lrange x 0 0"    => ["1"];
        "lrange x 0 10"   => ["1", "2", "3", "4"];
        "lrange x 0 -1"   => ["1", "2", "3", "4"];
        "lrange x 0 -2"   => ["1", "2", "3"];
    }
}
