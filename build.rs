use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("lib.rs");
    let mut f = File::create(&dest).unwrap();

    f.write_all(b"const CHESSGROUND: &str = \"CHESS\\\"GROUND\";").unwrap();
}
