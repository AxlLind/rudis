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
register_commands! {
    acl,                  // unimplemented
    append,
    asking,               // unimplemented
    auth,                 // unimplemented
    bgrewriteaof,         // unimplemented
    bgsave,               // unimplemented
    bitcount,             // unimplemented
    bitfield_ro,          // unimplemented
    bitfield,             // unimplemented
    bitop,                // unimplemented
    bitpos,               // unimplemented
    blmove,               // unimplemented
    blmpop,               // unimplemented
    blpop,                // unimplemented
    brpop,                // unimplemented
    brpoplpush,           // unimplemented
    bzmpop,               // unimplemented
    bzpopmax,             // unimplemented
    bzpopmin,             // unimplemented
    client,               // unimplemented
    cluster,              // unimplemented
    command,
    config,               // unimplemented
    copy,
    dbsize,
    debug,                // unimplemented
    decr,
    decrby,
    del,
    discard,              // unimplemented
    dump,                 // unimplemented
    echo,
    eval_ro,              // unimplemented
    eval,                 // unimplemented
    evalsha_ro,           // unimplemented
    evalsha,              // unimplemented
    exec,                 // unimplemented
    exists,
    expire,               // unimplemented
    expireat,             // unimplemented
    expiretime,           // unimplemented
    failover,             // unimplemented
    fcall_ro,             // unimplemented
    fcall,                // unimplemented
    flushall,
    flushdb,
    function,             // unimplemented
    geoadd,               // unimplemented
    geodist,              // unimplemented
    geohash,              // unimplemented
    geopos,               // unimplemented
    georadius_ro,         // unimplemented
    georadius,            // unimplemented
    georadiusbymember_ro, // unimplemented
    georadiusbymember,    // unimplemented
    geosearch,            // unimplemented
    geosearchstore,       // unimplemented
    get,
    getbit,               // unimplemented
    getdel,
    getex,                // unimplemented
    getrange,             // unimplemented
    getset,
    hdel,                 // unimplemented
    hello,                // unimplemented
    hexists,              // unimplemented
    hget,                 // unimplemented
    hgetall,              // unimplemented
    hincrby,              // unimplemented
    hincrbyfloat,         // unimplemented
    hkeys,                // unimplemented
    hlen,                 // unimplemented
    hmget,                // unimplemented
    hmset,                // unimplemented
    hrandfield,           // unimplemented
    hscan,                // unimplemented
    hset,                 // unimplemented
    hsetnx,               // unimplemented
    hstrlen,              // unimplemented
    hvals,                // unimplemented
    incr,
    incrby,
    incrbyfloat,          // unimplemented
    info,                 // unimplemented
    keys,                 // unimplemented
    lastsave,             // unimplemented
    latency,              // unimplemented
    lcs,                  // unimplemented
    lindex,               // unimplemented
    linsert,              // unimplemented
    llen,
    lmove,                // unimplemented
    lmpop,                // unimplemented
    lolwut,               // unimplemented
    lpop,
    lpos,                 // unimplemented
    lpush,
    lpushx,               // unimplemented
    lrange,
    lrem,                 // unimplemented
    lset,                 // unimplemented
    ltrim,                // unimplemented
    memory,               // unimplemented
    mget,                 // unimplemented
    migrate,              // unimplemented
    module,               // unimplemented
    monitor,              // unimplemented
    r#move,               // unimplemented
    mset,                 // unimplemented
    msetnx,               // unimplemented
    multi,                // unimplemented
    object,               // unimplemented
    persist,              // unimplemented
    pexpire,              // unimplemented
    pexpireat,            // unimplemented
    pexpiretime,          // unimplemented
    pfadd,                // unimplemented
    pfcount,              // unimplemented
    pfdebug,              // unimplemented
    pfmerge,              // unimplemented
    pfselftest,           // unimplemented
    ping,
    psetex,               // unimplemented
    psubscribe,           // unimplemented
    psync,                // unimplemented
    pttl,                 // unimplemented
    publish,              // unimplemented
    pubsub,               // unimplemented
    punsubscribe,         // unimplemented
    quit,
    randomkey,            // unimplemented
    readonly,             // unimplemented
    readwrite,            // unimplemented
    rename,
    renamenx,
    replconf,             // unimplemented
    replicaof,            // unimplemented
    reset,                // unimplemented
    restore_asking,       // unimplemented
    restore,              // unimplemented
    role,                 // unimplemented
    rpop,
    rpoplpush,            // unimplemented
    rpush,
    rpushx,               // unimplemented
    sadd,
    save,                 // unimplemented
    scan,                 // unimplemented
    scard,
    script,               // unimplemented
    sdiff,
    sdiffstore,
    select,               // unimplemented
    set,
    setbit,               // unimplemented
    setex,                // unimplemented
    setnx,                // unimplemented
    setrange,             // unimplemented
    shutdown,             // unimplemented
    sinter,
    sintercard,
    sinterstore,
    sismember,
    slaveof,              // unimplemented
    slowlog,              // unimplemented
    smembers,
    smismember,           // unimplemented
    smove,
    sort_ro,              // unimplemented
    sort,                 // unimplemented
    spop,                 // unimplemented
    spublish,             // unimplemented
    srandmember,          // unimplemented
    srem,
    sscan,                // unimplemented
    ssubscribe,           // unimplemented
    strlen,
    subscribe,            // unimplemented
    substr,               // unimplemented
    sunion,
    sunionstore,
    sunsubscribe,         // unimplemented
    swapdb,               // unimplemented
    sync,                 // unimplemented
    time,
    touch,                // unimplemented
    ttl,                  // unimplemented
    r#type,
    unlink,
    unsubscribe,          // unimplemented
    unwatch,              // unimplemented
    wait,                 // unimplemented
    watch,                // unimplemented
    xack,                 // unimplemented
    xadd,                 // unimplemented
    xautoclaim,           // unimplemented
    xclaim,               // unimplemented
    xdel,                 // unimplemented
    xgroup,               // unimplemented
    xinfo,                // unimplemented
    xlen,                 // unimplemented
    xpending,             // unimplemented
    xrange,               // unimplemented
    xread,                // unimplemented
    xreadgroup,           // unimplemented
    xrevrange,            // unimplemented
    xsetid,               // unimplemented
    xtrim,                // unimplemented
    zadd,                 // unimplemented
    zcard,                // unimplemented
    zcount,               // unimplemented
    zdiff,                // unimplemented
    zdiffstore,           // unimplemented
    zincrby,              // unimplemented
    zinter,               // unimplemented
    zintercard,           // unimplemented
    zinterstore,          // unimplemented
    zlexcount,            // unimplemented
    zmpop,                // unimplemented
    zmscore,              // unimplemented
    zpopmax,              // unimplemented
    zpopmin,              // unimplemented
    zrandmember,          // unimplemented
    zrange,               // unimplemented
    zrangebylex,          // unimplemented
    zrangebyscore,        // unimplemented
    zrangestore,          // unimplemented
    zrank,                // unimplemented
    zrem,                 // unimplemented
    zremrangebylex,       // unimplemented
    zremrangebyrank,      // unimplemented
    zremrangebyscore,     // unimplemented
    zrevrange,            // unimplemented
    zrevrangebylex,       // unimplemented
    zrevrangebyscore,     // unimplemented
    zrevrank,             // unimplemented
    zscan,                // unimplemented
    zscore,               // unimplemented
    zunion,               // unimplemented
    zunionstore,          // unimplemented
}
