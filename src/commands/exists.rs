use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct ExistsCommand;

impl RedisCommand for ExistsCommand {
    fn name(&self) -> &'static str {
        "exists"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let keys = cmd.parse_args::<Vec<ByteString>>()?;
        anyhow::ensure!(!keys.is_empty(), "expected EXISTS key [key ...]");
        Ok(Response::Number(keys.iter().filter(|&key| db.is_set(key)).count() as _))
    }
}
