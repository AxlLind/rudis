use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"strlen",
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
    Ok(match db.get_str(&key)? {
        Some(s) => Response::Number(s.len() as _),
        None => Response::Number(0),
    })
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_strlen
        "set x a"   => "OK";
        "set y abc" => "OK";
        "strlen x"  => 1;
        "strlen y"  => 3;
        "strlen q"  => 0;
    }
}
