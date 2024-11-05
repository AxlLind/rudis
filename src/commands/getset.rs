use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct GetsetCommand;

impl RedisCommand for GetsetCommand {
    fn name(&self) -> &'static str {
        "getset"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, value) = cmd.parse_args::<(ByteString, ByteString)>()?;
        Ok(match db.get_str(&key)? {
            Some(s) => Response::String(std::mem::replace(s, value)),
            None => Response::Nil,
        })
    }
}
