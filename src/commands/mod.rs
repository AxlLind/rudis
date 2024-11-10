use std::collections::HashMap;
use std::sync::LazyLock;

use crate::{Command, Database, Response, ByteString, Value};

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
    fn info(&self) -> &'static CommandInfo;
    fn run(&self, db: &mut Database, cmd: Command) -> anyhow::Result<Response>;
}

macro_rules! register_commands {
    ($($command:ident,)+) => {
        $(mod $command;)+
        pub const COMMAND_LIST: &[&dyn RedisCommand] = &[$(&$command::Cmd as _),+];
    };
}
register_commands!(
    append,
    command,
    copy,
    dbsize,
    decr,
    decrby,
    del,
    exists,
    flushall,
    flushdb,
    get,
    getdel,
    getset,
    incr,
    incrby,
    llen,
    lpop,
    lpush,
    lrange,
    ping,
    rename,
    renamenx,
    rpush,
    sadd,
    scard,
    sdiff,
    sdiffstore,
    set,
    sismember,
    smembers,
    strlen,
    type_,
    unlink,
);

pub static COMMANDS: LazyLock<HashMap<&[u8], &dyn RedisCommand>> = LazyLock::new(||
    COMMAND_LIST.iter().map(|&d| (d.info().name, d)).collect()
);
