use std::cmp::Ordering;
use crate::Response;

pub trait AsResponse: Sized {
    fn as_response(s: Self) -> Response;
}

impl AsResponse for () {
    fn as_response(_: Self) -> Response { Response::Nil }
}

impl AsResponse for &str {
    fn as_response(s: Self) -> Response { Response::BulkString(s.to_string().into_bytes()) }
}

impl AsResponse for i64 {
    fn as_response(s: Self) -> Response { Response::Number(s) }
}

impl AsResponse for f64 {
    fn as_response(s: Self) -> Response { Response::BulkString(s.to_string().into_bytes()) }
}

pub fn _sort_response(r: &mut Response) {
    let Response::Array(v) = r else { return };
    v.sort_by(|a, b| match (a, b) {
        (Response::Number(x), Response::Number(y)) => x.cmp(y),
        (Response::SimpleString(x), Response::SimpleString(y)) => x.cmp(y),
        (Response::BulkString(x), Response::BulkString(y)) => x.cmp(y),
        _ => Ordering::Equal,
    });
}

#[macro_export]
macro_rules! command_test {
    (@expand_test $db:ident $cmd:literal => $expected:expr; $($rest:tt)*) => {
        let cmd = $crate::Command::new($cmd.split(' ').map(|w| w.as_bytes().to_vec()).collect()).unwrap();
        let res = $crate::execute_command(&mut $db, cmd).unwrap();
        let expected = $expected;
        match (res, expected) {
            (Response::SimpleString(res), Response::BulkString(expected)) => {
                assert_eq!(Response::SimpleString(res), Response::SimpleString(expected), $cmd);
            }
            (res, expected) => assert_eq!(res, expected, $cmd),
        }
        $crate::command_test!{ @expand_tests $db $($rest)* }
    };
    (@expand_tests $db:ident $cmd:literal => [$($expected:expr),* $(,)?] ignore_order; $($rest:tt)*) => {
        let cmd = $crate::Command::new($cmd.split(' ').map(|w| w.as_bytes().to_vec()).collect()).unwrap();
        let mut res = $crate::execute_command(&mut $db, cmd).unwrap();
        let mut expected = Response::Array(vec![$($crate::test_utils::AsResponse::as_response($expected)),*]);
        $crate::test_utils::_sort_response(&mut res);
        $crate::test_utils::_sort_response(&mut expected);
        assert_eq!(res, expected, $cmd);
        $crate::command_test!{ @expand_tests $db $($rest)* }
    };
    (@expand_tests $db:ident $cmd:literal => [$($expected:expr),* $(,)?]; $($rest:tt)*) => {
        $crate::command_test!(@expand_test $db $cmd => Response::Array(vec![$($crate::test_utils::AsResponse::as_response($expected)),*]); $($rest)*);
    };
    (@expand_tests $db:ident $cmd:literal => $expected:expr; $($rest:tt)*) => {
        $crate::command_test!(@expand_test $db $cmd => $crate::test_utils::AsResponse::as_response($expected); $($rest)*);
    };
    (@expand_tests $db:ident) => {};

    ($($tokens:tt)*) => {
        #[test]
        fn test_cmd() {
            let mut db = $crate::Database::default();
            $crate::command_test!(@expand_tests db $($tokens)*);
        }
    };
}
