use super::RedisCommand;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub struct LrangeCommand;

impl RedisCommand for LrangeCommand {
    fn name(&self) -> &'static str {
        "lrange"
    }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, start, stop) = cmd.parse_args::<(ByteString, i64, i64)>()?;
        Ok(match db.get_list(&key)? {
            Some(list) => {
                let start = if start < 0 {list.len() - 2 - start as usize} else {start as usize};
                let stop = if stop < 0 {list.len() - 2 - stop as usize} else {stop as usize};
                // TODO: Implement more correct index handling here
                Response::List(list[start..=stop].iter().cloned().collect())
            }
            None => Response::List(Vec::new()),
        })
    }
}
