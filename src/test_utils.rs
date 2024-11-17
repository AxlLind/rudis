use crate::Response;

pub trait AsResponse {
    fn as_response(s: Self) -> Response;
}

impl AsResponse for () {
    fn as_response(_: Self) -> Response { Response::Nil }
}

impl AsResponse for &str {
    fn as_response(s: Self) -> Response { Response::SimpleString(s.to_string().into_bytes()) }
}

impl AsResponse for i64 {
    fn as_response(s: Self) -> Response { Response::Number(s) }
}

impl<const N: usize> AsResponse for [&str; N] {
    fn as_response(s: Self) -> Response {
        Response::string_array(s.iter().map(|x| x.as_bytes().to_vec()).collect())
    }
}

#[macro_export]
macro_rules! command_test {
    ($($cmd:literal => $expected:expr;)+) => {
        #[test]
        fn test_cmd() {
            let mut db = $crate::Database::default();
            $(
                let cmd = $crate::Command::new($cmd.split(' ').map(|w| w.as_bytes().to_vec()).collect()).unwrap();
                let res = $crate::execute_command(&mut db, cmd).unwrap();
                let expected = $crate::test_utils::AsResponse::as_response($expected);
                assert_eq!(res, expected, $cmd);
            )+
        }
    };
}
