use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct RenameCommand;

impl RedisCommand for RenameCommand {
    fn name(&self) -> &'static str {
        "rename"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, newkey) = cmd.parse_args::<(ByteString, ByteString)>()?;
        let val = db.del(&key).ok_or(anyhow::anyhow!("key does not exist"))?;
        db.set(newkey, val);
        Ok(Response::String(b"OK".to_vec()))
    }
}
