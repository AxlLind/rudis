use super::RedisCommand;
use crate::command::Command;
use crate::{Database, Response};

pub struct CommandCommand;

impl RedisCommand for CommandCommand {
    fn name(&self) -> &'static str {
        "command"
    }

    fn run(&self, db: &mut Database, cmd: Command) -> anyhow::Result<Response> {
        let _ = (db, cmd);
        // TODO: Implement this somehow
        Ok(Response::List(vec![]))
    }
}
