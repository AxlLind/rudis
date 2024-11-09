use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

static INFO: CommandInfo = CommandInfo {
    name: b"sadd",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
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
        let (key, elems) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
        anyhow::ensure!(!elems.is_empty(), "expected SADD member [member ...]");
        match db.get_set(&key)? {
            Some(set) => {
                let prelen = set.len();
                set.extend(elems.into_iter());
                Ok(Response::Number((set.len() - prelen) as _))
            }
            None => {
                let len = elems.len();
                db.set(key, Value::Set(elems.into_iter().collect()));
                Ok(Response::Number(len as _))
            }
        }
    }
}
