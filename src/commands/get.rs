use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct GetCommand;

impl RedisCommand for GetCommand {
    fn name(&self) -> &'static str {
        "get"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let key = cmd.parse_args::<ByteString>()?;
        Ok(match db.get_str(&key)? {
            Some(s) => Response::String(s.clone()),
            None => Response::Nil,
        })
    }
}
