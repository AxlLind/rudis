use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"ping",
    arity: -1,
    flags: &[
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(_: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let message = cmd.parse_args::<Option<ByteString>>()?;
    Ok(Response::String(message.unwrap_or_else(|| b"PONG".to_vec())))
}

#[cfg(test)]
crate::command_test! {
    "ping"      => "PONG";
    "ping PONG" => "PONG";
    "ping HI"   => "HI";
}
