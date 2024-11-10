use super::{incr_by, CommandInfo};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"incrby",
    arity: 3,
    flags: &[
        b"write",
        b"denyoom",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, step) = cmd.parse_args::<(ByteString, i64)>()?;
    incr_by(db, key, step)
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_incrby
        "incrby x 10"   => 10;
        "incrby x -10"  => 0;
        "set x 234"     => "OK";
        "incrby x 1000" => 1234;
        "get x"         => "1234";
    }
}
