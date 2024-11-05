use super::RedisCommand;
use crate::command::Command;
use crate::{Value, ByteString, Database, Response};

pub struct LpushCommand;

impl RedisCommand for LpushCommand {
    fn name(&self) -> &'static str {
        "lpush"
    }

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
                db.set(key, Value::List(elements));
                Response::Number(len as _)
            }
        })
    }
}
