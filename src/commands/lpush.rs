use super::{CommandInfo, RedisCommand};
use crate::cmd_parser::Command;
use crate::{Value, ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"lpush",
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
        let (key, elements) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
        anyhow::ensure!(!elements.is_empty(), "expected LPUSH key element [element ...]");
        Ok(match db.get_list(&key)? {
            Some(list) => {
                for (i, e) in elements.into_iter().enumerate() {
                    list.insert(i, e);
                }
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
