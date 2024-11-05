use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct RenamenxCommand;

impl RedisCommand for RenamenxCommand {
    fn name(&self) -> &'static str {
        "renamenx"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
        let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
        let n = if db.is_set(&newkey) {
            0
        } else {
            db.set(newkey, val);
            1
        };
        Ok(Response::Number(n))
    }
}
