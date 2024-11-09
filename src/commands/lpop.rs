use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"lpop",
    arity: -2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, count) = cmd.parse_args::<(ByteString, Option<i64>)>()?;
        let Some(list) = db.get_list(&key)? else { return Ok(Response::Nil) };
        Ok(match count {
            Some(n) if n < 0 => anyhow::bail!("value is out of range, must be positive"),
            Some(n) => {
                let n = list.len().min(n as _);
                let mut x = list.split_off(n);
                std::mem::swap(&mut x, list);
                Response::Array(x)
            }
            None => Response::String(list.remove(0)),
        })
    }
}
