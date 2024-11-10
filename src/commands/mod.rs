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

type CommandFn = fn(&mut Database, Command) -> anyhow::Result<Response>;

macro_rules! register_commands {
    ($($command:ident,)+) => {
        $(mod $command;)+

        pub static COMMAND_LIST: &[(CommandFn, &CommandInfo)] = &[$(($command::run, &$command::INFO)),+];

        pub static COMMANDS: LazyLock<HashMap<&[u8], CommandFn>> = LazyLock::new(|| {
            let mut commands = HashMap::new();
            $(commands.insert($command::INFO.name, $command::run as _);)+
            commands
        });
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
    echo,
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
    quit,
    rename,
    renamenx,
    rpop,
    rpush,
    sadd,
    scard,
    sdiff,
    sdiffstore,
    set,
    sinter,
    sintercard,
    sinterstore,
    sismember,
    smembers,
    smove,
    srem,
    strlen,
    sunion,
    sunionstore,
    time,
    type_,
    unlink,
);
