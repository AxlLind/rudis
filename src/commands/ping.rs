use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"ping",
    arity: -1,
    flags: &[
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, _: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let message = cmd.parse_args::<Option<ByteString>>()?;
        Ok(Response::String(message.unwrap_or_else(|| b"PONG".to_vec())))
    }
}
