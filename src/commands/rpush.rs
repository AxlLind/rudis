use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{Value, ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"rpush",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct RpushCommand;

impl RedisCommand for RpushCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, elements) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
        anyhow::ensure!(!elements.is_empty(), "expected RPUSH key element [element ...]");
        Ok(match db.get_list(&key)? {
            Some(list) => {
                list.extend(elements);
                Response::Number(list.len() as _)
            }
            None => {
                let len = elements.len();
                db.set(key, Value::Array(elements));
                Response::Number(len as _)
            }
        })
    }
}
