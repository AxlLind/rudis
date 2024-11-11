use super::{clamp_range, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"substr",
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
    Ok(match db.get_str(&key)? {
        Some(list) => {
            let (start, stop) = clamp_range(list.len(), start, stop);
            Response::String(list[start..=stop].to_vec())
        }
        None => Response::String(Vec::new()),
    })
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_strlen
        "set x this_is_a_string" => "OK";
        "substr x 0 3"           => "this";
        "substr x -3 -1"         => "ing";
        "substr x 0 -1"          => "this_is_a_string";
        "substr x 10 100"        => "string";
        "substr q 0 -1"          => "";
    }
}
