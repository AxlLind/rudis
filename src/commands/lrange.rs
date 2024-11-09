use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"lrange",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct LrangeCommand;

impl RedisCommand for LrangeCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, start, stop) = cmd.parse_args::<(ByteString, i64, i64)>()?;
        Ok(match db.get_list(&key)? {
            Some(list) => {
                let start = if start < 0 {list.len() - 2 - start as usize} else {start as usize};
                let stop = if stop < 0 {list.len() - 2 - stop as usize} else {stop as usize};
                // TODO: Implement more correct index handling here
                Response::Array(list[start..=stop].iter().cloned().collect())
            }
            None => Response::Array(Vec::new()),
        })
    }
}
