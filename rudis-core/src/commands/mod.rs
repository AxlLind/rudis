use std::collections::HashMap;
use std::sync::LazyLock;

use crate::{Command, Database, Response, ByteString, Value};

pub fn int_from_bytes(bytes: &[u8]) -> anyhow::Result<i64> {
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

pub fn clamp_range(max: usize, start: i64, stop: i64) -> (usize, usize) {
    fn clamp_index(max: usize, i: i64) -> usize {
        let x = if i < 0 { max as i64 + i } else { i };
        x.clamp(0, max as i64 - 1) as _
    }
    (clamp_index(max, start), clamp_index(max, stop))
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

impl CommandInfo {
    pub fn as_response(&self) -> Response {
        Response::Array(vec![
            Response::BulkString(self.name.to_vec()),
            Response::Number(self.arity),
            Response::string_array(self.flags.iter().map(|s| s.to_vec())),
            Response::Number(self.first_key),
            Response::Number(self.last_key),
            Response::Number(self.step),
        ])
    }
}

type CommandFn = fn(&mut Database, Command) -> anyhow::Result<Response>;

macro_rules! register_commands {
    ($($command:ident,)+) => {
        $(mod $command;)+

        pub static COMMAND_LIST: &[(CommandFn, &CommandInfo)] = &[$(($command::run, &$command::INFO)),+];

        pub static COMMANDS: LazyLock<HashMap<&[u8], (CommandFn, &CommandInfo)>> = LazyLock::new(|| {
            let mut commands = HashMap::new();
            $(commands.insert($command::INFO.name, ($command::run as _, &$command::INFO));)+
            commands
        });
    };
}
register_commands! {
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
    getbit,
    getdel,
    getrange,
    getset,
    hdel,
    hexists,
    hget,
    hgetall,
    hincrby,
    hkeys,
    hlen,
    hset,
    hstrlen,
    hvals,
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
    setbit,
    sinter,
    sintercard,
    sinterstore,
    sismember,
    smembers,
    smove,
    srem,
    strlen,
    substr,
    sunion,
    sunionstore,
    time,
    r#type,
    unlink,
    zadd,
    zcard,
    zcount,
    zrem,
    zscore,
}
