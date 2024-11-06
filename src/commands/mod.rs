use std::collections::HashMap;
use std::sync::LazyLock;

use crate::{Command, Database, Response, ByteString, Value};

macro_rules! register_commands {
    ($($m:ident::$c:ident,)+) => {
        $(mod $m;)+
        pub const COMMAND_LIST: &[&dyn RedisCommand] = &[$(&$m::$c as _,)+];
    };
}
register_commands!(
    append::AppendCommand,
    command::CommandCommand,
    copy::CopyCommand,
    decr::DecrCommand,
    decrby::DecrbyCommand,
    del::DelCommand,
    exists::ExistsCommand,
    flushall::FlushallCommand,
    get::GetCommand,
    getdel::GetdelCommand,
    getset::GetsetCommand,
    incr::IncrCommand,
    incrby::IncrbyCommand,
    lpush::LpushCommand,
    lrange::LrangeCommand,
    ping::PingCommand,
    rename::RenameCommand,
    renamenx::RenamenxCommand,
    set::SetCommand,
    strlen::StrlenCommand,
    type_::TypeCommand,
    unlink::UnlinkCommand,
);

fn int_from_bytes(bytes: &[u8]) -> anyhow::Result<i64> {
    std::str::from_utf8(bytes)
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-utf8 value"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("tried to parse number, got non-numeric value"))
}

pub fn incr_by(db: &mut Database, key: ByteString, step: i64) -> anyhow::Result<Response> {
    let val = step + match db.get_str(&key)? {
        Some(v) => int_from_bytes(v)?,
        None => 0,
    };
    db.set(key, Value::String(val.to_string().into_bytes()));
    Ok(Response::Number(val))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInfo {
    pub name: &'static [u8],
    pub arity: i64,
    pub flags: &'static [&'static [u8]],
    pub first_key: i64,
    pub last_key: i64,
    pub step: i64,
}

pub trait RedisCommand: Send + Sync {
    fn name(&self) -> &'static [u8];
    fn info(&self) -> &'static CommandInfo;
    fn run(&self, db: &mut Database, cmd: Command) -> anyhow::Result<Response>;
}

pub static COMMANDS: LazyLock<HashMap<&[u8], &dyn RedisCommand>> = LazyLock::new(||
    COMMAND_LIST.iter().map(|&d| (d.name(), d)).collect::<HashMap<_,_>>()
);
