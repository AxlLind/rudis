use redis_in_rust::{Database, Response, Command, execute_command};

trait AsResponse {
    fn as_response(s: Self) -> Response;
}

impl AsResponse for &str {
    fn as_response(s: Self) -> Response { Response::String(s.to_string().into_bytes()) }
}

impl AsResponse for Vec<u8> {
    fn as_response(s: Self) -> Response { Response::String(s) }
}

impl AsResponse for i64 {
    fn as_response(s: Self) -> Response { Response::Number(s) }
}

impl<const N: usize> AsResponse for [&str; N] {
    fn as_response(s: Self) -> Response {
        Response::Array(s.iter().map(|x| x.as_bytes().to_vec()).collect())
    }
}

struct Nil;

impl AsResponse for Nil {
    fn as_response(_: Self) -> Response { Response::Nil }
}

macro_rules! compatability_tests {
    ($id:ident : { commands: [$($cmd:expr,)+], results: [$($res:expr,)+], since: $since:literal $(,)?}, $($rest:tt)*) => {
        #[test]
        fn $id() -> anyhow::Result<()> {
            let mut db = Database::default();
            let results = [
                $({
                    let cmd = Command::new($cmd.split(' ').map(|w| w.as_bytes().to_vec()).collect()).unwrap();
                    execute_command(&mut db, cmd).unwrap()
                },)+
            ];
            let expected = [$(AsResponse::as_response($res)),+];
            assert_eq!(results, expected);
            Ok(())
        }
        compatability_tests! { $($rest)* }
    };
    () => {};
}

compatability_tests! {
    test_del_command: {
        commands: [
            "set k v",
            "del k",
        ],
        results: [
            "OK",
            1,
        ],
        since: "1.0.0",
    },
    test_rename_command: {
        commands: [
            "set k v",
            "rename k kk",
        ],
        results: [
            "OK",
            "OK",
        ],
        since: "1.0.0",
    },
    test_rename_command2: {
        commands: [
            "set k v",
            "rename k {k}k",
        ],
        results: [
            "OK",
            "OK",
        ],
        since: "1.0.0",
    },
    test_renamenx_command: {
        commands: [
            "set k v",
            "renamenx k kk",
        ],
        results: [
            "OK",
            1,
        ],
        since: "1.0.0",
    },
    test_renamenx_command2: {
        commands: [
            "set k v",
            "renamenx k {k}k",
        ],
        results: [
            "OK",
            1,
        ],
        since: "1.0.0",
    },
    test_randomkey_command: {
        commands: [
            "set k v",
            "randomkey",
        ],
        results: [
            "OK",
            "k",
        ],
        since: "1.0.0",
    },
    test_exists_command: {
        commands: [
            "set k v",
            "exists k",
        ],
        results: [
            "OK",
            1,
        ],
        since: "1.0.0",
    },
    test_ttl_command: {
        commands: [
            "ttl non-exists",
        ],
        results: [
            -2,
        ],
        since: "1.0.0",
    },
    test_pttl_command: {
        commands: [
            "pttl non-exists",
        ],
        results: [
            -2,
        ],
        since: "1.0.0",
    },
    test_expire_command: {
        commands: [
            "expire non-exists 10",
        ],
        results: [
            0,
        ],
        since: "1.0.0",
    },
    test_keys_command: {
        commands: [
            "mset firstname Jack lastname Stuntman age 35",
            "keys a??",
        ],
        results: [
            "OK",
            ["age"],
        ],
        since: "1.0.0",
    },
    test_move_command: {
        commands: [
            "set k v",
            "move k 1",
        ],
        results: [
            "OK",
            1,
        ],
        since: "1.0.0",
    },
    test_type_command: {
        commands: [
            "set k v",
            "type k",
        ],
        results: [
            "OK",
            "string",
        ],
        since: "1.0.0",
    },
    test_sort_command: {
        commands: [
            "lpush list 5 3 4 1 2",
            "sort list",
        ],
        results: [
            5,
            ["1", "2", "3", "4", "5"],
        ],
        since: "1.0.0",
    },
    test_set_command: {
        commands: [
            "set k v",
        ],
        results: [
            "OK",
        ],
        since: "1.0.0",
    },
    test_lindex_command: {
        commands: [
            "lpush mylist 0",
            "lpush mylist 1",
            "lindex mylist -1",
        ],
        results: [
            1,
            2,
            "0",
        ],
        since: "1.0.0",
    },
    test_llen_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "llen mylist",
        ],
        results: [
            1,
            2,
            3,
            3,
        ],
        since: "1.0.0",
    },
    test_lpop_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "rpush mylist 3",
            "rpush mylist 4",
            "lpop mylist 2",
            "lpop mylist 3",
        ],
        results: [
            1,
            2,
            3,
            4,
            5,
            ["0", "1"],
            ["2", "3", "4"],
        ],
        since: "1.0.0",
    },
    test_lpush_command: {
        commands: [
            "lpush mylist 0",
            "lpush mylist 1",
            "lrange mylist 0 -1",
        ],
        results: [
            1,
            2,
            ["1", "0"],
        ],
        since: "1.0.0",
    },
    test_lrange_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "lrange mylist 0 -1",
        ],
        results: [
            1,
            2,
            3,
            ["0", "1", "2"],
        ],
        since: "1.0.0",
    },
    test_lrem_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "rpush mylist 3",
            "rpush mylist 3",
            "rpush mylist 3",
            "rpush mylist 4",
            "rpush mylist 5",
            "lrem mylist -1 0",
            "lrem mylist 1 2",
            "lrem mylist 0 3",
        ],
        results: [
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            1,
            1,
            3,
        ],
        since: "1.0.0",
    },
    test_lset_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "lset mylist 0 3",
            "lrange mylist 0 -1",
        ],
        results: [
            1,
            2,
            3,
            "OK",
            ["3", "1", "2"],
        ],
        since: "1.0.0",
    },
    test_ltrim_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "ltrim mylist 1 -1",
            "lrange mylist 0 -1",
        ],
        results: [
            1,
            2,
            3,
            "OK",
            ["1", "2"],
        ],
        since: "1.0.0",
    },
    test_rpop_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "rpush mylist 2",
            "rpush mylist 3",
            "rpush mylist 4",
            "rpop mylist",
        ],
        results: [
            1,
            2,
            3,
            4,
            5,
            "4",
        ],
        since: "1.0.0",
    },
    test_rpush_command: {
        commands: [
            "rpush mylist 0",
            "rpush mylist 1",
            "lrange mylist 0 -1",
        ],
        results: [
            1,
            2,
            ["0", "1"],
        ],
        since: "1.0.0",
    },
    test_sadd_command: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd myset 0",
        ],
        results: [
            1,
            1,
            0,
        ],
        since: "1.0.0",
    },
    test_scard_command: {
        commands: [
            "scard myset",
            "sadd myset 0",
            "sadd myset 1",
            "scard myset",
        ],
        results: [
            0,
            1,
            1,
            2,
        ],
        since: "1.0.0",
    },
    test_sdiff_command: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd myset 2",
            "sadd myset1 2",
            "sadd myset1 3",
            "sadd myset1 4",
            "sadd myset2 1",
            "sadd myset2 7",
            "sdiff myset myset1 myset2",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            ["0"],
        ],
        since: "1.0.0",
    },
    test_sdiff_command2: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd myset 2",
            "sadd {myset}1 2",
            "sadd {myset}1 3",
            "sadd {myset}1 4",
            "sdiff myset {myset}1",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
            1,
            ["0", "1"],
        ],
        since: "1.0.0",
    },
    test_sdiffstore_command: {
        commands: [
            "sadd myset1 0",
            "sadd myset1 1",
            "sadd myset1 2",
            "sadd myset2 2",
            "sadd myset2 3",
            "sadd myset2 4",
            "sdiffstore myset myset1 myset2",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
            1,
            2,
        ],
        since: "1.0.0",
    },
    test_sdiffstore_command2: {
        commands: [
            "sadd {myset}1 0",
            "sadd {myset}1 1",
            "sadd {myset}1 2",
            "sadd {myset}2 2",
            "sadd {myset}2 3",
            "sadd {myset}2 4",
            "sdiffstore myset {myset}1 {myset}2",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
            1,
            2,
        ],
        since: "1.0.0",
    },
    test_sinter_command: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd myset1 1",
            "sadd myset1 2",
            "sinter myset myset1",
        ],
        results: [
            1,
            1,
            1,
            1,
            ["1"],
        ],
        since: "1.0.0",
    },
    test_sinter_command2: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd {myset}1 1",
            "sadd {myset}1 2",
            "sinter myset {myset}1",
        ],
        results: [
            1,
            1,
            1,
            1,
            ["1"],
        ],
        since: "1.0.0",
    },
    test_sintercard_command: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd myset 2",
            "sadd myset1 1",
            "sadd myset1 2",
            "sadd myset1 3",
            "sintercard myset myset1",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
            1,
            2,
        ],
        since: "1.0.0",
    },
    test_sinterstore_command: {
        commands: [
            "sadd myset1 0",
            "sadd myset1 1",
            "sadd myset2 1",
            "sadd myset2 2",
            "sinterstore myset myset1 myset2",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
        ],
        since: "1.0.0",
    },
    test_sinterstore_command2: {
        commands: [
            "sadd {myset}1 0",
            "sadd {myset}1 1",
            "sadd {myset}2 1",
            "sadd {myset}2 2",
            "sinterstore myset {myset}1 {myset}2",
        ],
        results: [
            1,
            1,
            1,
            1,
            1,
        ],
        since: "1.0.0",
    },
    test_sismember_command: {
        commands: [
            "sadd myset 0",
            "sismember myset 0",
            "sismember myset 1",
        ],
        results: [
            1,
            1,
            0,
        ],
        since: "1.0.0",
    },
    test_smembers_command: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "smembers myset",
        ],
        results: [
            1,
            1,
            ["0", "1"],
        ],
        since: "1.0.0",
    },
    test_smove_command: {
        commands: [
            "sadd myset 0",
            "sadd myset 1",
            "sadd myotherset 2",
            "smove myset myotherset 1",
        ],
        results: [
            1,
            1,
            1,
            1,
        ],
        since: "1.0.0",
    },
    test_smove_command2: {
        commands: [
            "sadd myset 0",
            "smove myset myset2 0",
            "smove myset myset2 0",
            "scard myset2",
        ],
        results: [
            1,
            1,
            0,
            1,
        ],
        since: "1.0.0",
    },
    test_spop_command: {
        commands: [
            "sadd myset 0",
            "spop myset",
        ],
        results: [
            1,
            "0",
        ],
        since: "1.0.0",
    },
    test_srandmember_command: {
        commands: [
            "sadd myset 0",
            "srandmember myset",
        ],
        results: [
            1,
            "0",
        ],
        since: "1.0.0",
    },
    test_srem_command: {
        commands: [
            "sadd myset 0 1 2",
            "srem myset 0 3 4",
            "srem myset 2",
            "scard myset",
        ],
        results: [
            3,
            1,
            1,
            1,
        ],
        since: "1.0.0",
    },
    test_sunion_command: {
        commands: [
            "sadd myset 0",
            "sadd myset1 1",
            "sunion myset myset1",
        ],
        results: [
            1,
            1,
            ["0", "1"],
        ],
        since: "1.0.0",
    },
    test_sunion_command2: {
        commands: [
            "sadd myset 0",
            "sadd {myset}1 1",
            "sunion myset {myset}1",
        ],
        results: [
            1,
            1,
            ["0", "1"],
        ],
        since: "1.0.0",
    },
    test_sunionstore_command: {
        commands: [
            "sadd myset1 0",
            "sadd myset2 1",
            "sunionstore myset myset1 myset2",
        ],
        results: [
            1,
            1,
            2,
        ],
        since: "1.0.0",
    },
    test_sunionstore_command2: {
        commands: [
            "sadd {myset}1 0",
            "sadd {myset}2 1",
            "sunionstore myset {myset}1 {myset}2",
        ],
        results: [
            1,
            1,
            2,
        ],
        since: "1.0.0",
    },
    test_decr_command: {
        commands: [
            "set mykey 10",
            "decr mykey",
        ],
        results: [
            "OK",
            9,
        ],
        since: "1.0.0",
    },
    test_decrby_command: {
        commands: [
            "set mykey 10",
            "decrby mykey 3",
        ],
        results: [
            "OK",
            7,
        ],
        since: "1.0.0",
    },
    test_get_command: {
        commands: [
            "set mykey 10",
            "get mykey",
        ],
        results: [
            "OK",
            "10",
        ],
        since: "1.0.0",
    },
    test_getset_command: {
        commands: [
            "set mylist abcd",
            "getset mylist bcde",
            "get mylist",
        ],
        results: [
            "OK",
            "abcd",
            "bcde",
        ],
        since: "1.0.0",
    },
    test_incr_command: {
        commands: [
            "set mykey 10",
            "incr mykey",
            "get mykey",
        ],
        results: [
            "OK",
            11,
            "11",
        ],
        since: "1.0.0",
    },
    test_incrby_command: {
        commands: [
            "set mykey 10",
            "incrby mykey 5",
            "get mykey",
        ],
        results: [
            "OK",
            15,
            "15",
        ],
        since: "1.0.0",
    },
    // TODO: Support different types in same array
    /*
    test_mget_command: {
        commands: [
            "set mykey0 1",
            "set mykey1 2",
            "mget mykey0 mykey1 mykey2",
        ],
        results: [
            "OK",
            "OK",
            ["1", "2", Nil],
        ],
        since: "1.0.0",
    },
    test_mget_command2: {
        commands: [
            "set {mykey}0 1",
            "set {mykey}1 2",
            "mget {mykey}0 {mykey}1 {mykey}2",
        ],
        results: [
            "OK",
            "OK",
            ["1", "2", Nil],
        ],
        since: "1.0.0",
    },
    */
    test_set_command2: {
        commands: [
            "set mykey 0",
            "get mykey",
        ],
        results: [
            "OK",
            "0",
        ],
        since: "1.0.0",
    },
    test_setnx_command: {
        commands: [
            "setnx mykey0 0",
            "setnx mykey0 1",
        ],
        results: [
            1,
            0,
        ],
        since: "1.0.0",
    },
    test_substr_command: {
        commands: [
            "set mykey 012",
            "substr mykey 0 -1",
        ],
        results: [
            "OK",
            "012",
        ],
        since: "1.0.0",
    },
    test_dbsize_command: {
        commands: [
            "dbsize",
        ],
        results: [
            0,
        ],
        since: "1.0.0",
    },
    test_flushall_command: {
        commands: [
            "set a b",
            "flushall",
            "get a",
        ],
        results: [
            "OK",
            "OK",
            Nil,
        ],
        since: "1.0.0",
    },
    test_flushdb_command: {
        commands: [
            "set a b",
            "flushdb",
            "get a",
        ],
        results: [
            "OK",
            "OK",
            Nil,
        ],
        since: "1.0.0",
    },
}
