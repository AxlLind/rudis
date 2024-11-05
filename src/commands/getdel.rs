use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct GetdelCommand;

impl RedisCommand for GetdelCommand {
    fn name(&self) -> &'static str {
        "getdel"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (src, dst) = cmd.parse_args::<(ByteString, ByteString)>()?;
        let value = match db.get(&src) {
            Some(v) => {
                let copy = v.clone();
                db.set(dst, copy);
                Response::Number(1)
            }
            None => Response::Number(0),
        };
        Ok(value)
    }
}
