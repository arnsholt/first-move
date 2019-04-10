use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("gen.rs");
    let mut f = File::create(&dest).unwrap();

    write_file(&mut f, "CHESSGROUND", "chessground.js");
    write_file(&mut f, "BOARD", "svg/blue.svg");
    write_file(&mut f, "WP", "svg/wP.svg");
    write_file(&mut f, "WB", "svg/wB.svg");
    write_file(&mut f, "WN", "svg/wN.svg");
    write_file(&mut f, "WR", "svg/wR.svg");
    write_file(&mut f, "WQ", "svg/wQ.svg");
    write_file(&mut f, "WK", "svg/wK.svg");
    write_file(&mut f, "BP", "svg/bP.svg");
    write_file(&mut f, "BB", "svg/bB.svg");
    write_file(&mut f, "BN", "svg/bN.svg");
    write_file(&mut f, "BR", "svg/bR.svg");
    write_file(&mut f, "BQ", "svg/bQ.svg");
    write_file(&mut f, "BK", "svg/bK.svg");
    write_file(&mut f, "ARROWS", "arrows.js");
}

fn write(f: &mut File, name: &str, value: &str) {
    f.write_all(format!("const {}: &str = \"{}\";", name, value).as_bytes()).unwrap();
}

fn write_file(f: &mut File, name: &str, infile: &str) {
    let s = read_to_string(infile).unwrap().replace('"', "\\\"");
    write(f, name, &s);
}
