use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct StrlenCommand;

impl RedisCommand for StrlenCommand {
    fn name(&self) -> &'static str {
        "strlen"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let key = cmd.parse_args::<ByteString>()?;
        Ok(match db.get_str(&key)? {
            Some(s) => Response::Number(s.len() as _),
            None => Response::Number(0),
        })
    }
}
