use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"echo",
    arity: 2,
    flags: &[
        b"loading",
        b"stale",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let msg = cmd.parse_args::<ByteString>()?;
    Ok(Response::String(msg))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_echo
        "echo hello" => "hello";
        "echo HELLO" => "HELLO";
    }
}
