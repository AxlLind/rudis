use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct UnlinkCommand;

impl RedisCommand for UnlinkCommand {
    fn name(&self) -> &'static str {
        "unlink"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let keys = cmd.parse_args::<Vec<ByteString>>()?;
        anyhow::ensure!(!keys.is_empty(), "expected UNLINK key [key ...]");
        Ok(Response::Number(keys.iter().filter(|&key| db.del(key).is_some()).count() as _))
    }
}
