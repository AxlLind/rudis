use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"getset",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, value) = cmd.parse_args::<(ByteString, ByteString)>()?;
        Ok(match db.get_str(&key)? {
            Some(s) => Response::String(std::mem::replace(s, value)),
            None => Response::Nil,
        })
    }

}
