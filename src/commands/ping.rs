use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct PingCommand;

impl RedisCommand for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn run(&self, _: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let message = cmd.parse_args::<Option<ByteString>>()?;
        Ok(Response::String(message.unwrap_or_else(|| b"PONG".to_vec())))
    }
}
