use std::collections::HashMap;
use std::sync::LazyLock;

use crate::{Command, Database, Response, ByteString, Value};

macro_rules! register_commands {
    ($($m:ident::$c:ident,)+) => {
        $(mod $m;)+
        const COMMAND_LIST: &[&dyn RedisCommand] = &[$(&$m::$c as _,)+];
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
    _type::TypeCommand,
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


pub trait RedisCommand: Send + Sync {
    fn run(&self, db: &mut Database, cmd: Command) -> anyhow::Result<Response>;

    fn name(&self) -> &'static str;
}

pub static COMMANDS: LazyLock<HashMap<&str, &dyn RedisCommand>> = LazyLock::new(||
    COMMAND_LIST.iter().map(|&d| (d.name(), d)).collect::<HashMap<_,_>>()
);
