include!(concat!(env!("OUT_DIR"), "/lib.rs"));
const STYLE: &str = "STYLE";

pub fn chessground<'a>() -> &'a str { CHESSGROUND }
pub fn style<'a>() -> &'a str { STYLE }
