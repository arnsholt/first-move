#![deny(clippy::all, clippy::pedantic)]
#![feature(proc_macro_hygiene)]

// External dependencies:
extern crate clap;
extern crate maud;
extern crate pgn_reader;
extern crate shakmaty;

use clap::{App, Arg};
use maud::html;
use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::{Chess, Color, Move, Position, Role, Setup, Square};
use shakmaty::fen::Fen;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator; // Not used directly, but enables .from_iter().

use first_move;
use first_move::{chessground,style};

struct Search<'a> {
    input: &'a str,
}

impl<'a> Search<'a> {
    fn new(input: &'a str) -> Search<'a> {
        Search { input }
    }

    fn run(&mut self, target: &str) {
        let fen: Fen = target.parse().unwrap();
        let target: Chess = fen.position().unwrap();
        println!("Searching from:\n{:?}", target.board());

        let mut visitor = PgnSearcher::new(target);
        let mut reader = BufferedReader::new(::std::fs::File::open(&self.input).expect("Error opening file"));
        reader.read_all(&mut visitor).expect("Error while parsing file");

        visitor.counts.iter().for_each(|(k, v)| Self::output(visitor.target.board().color_at(*k).unwrap(), visitor.target.board().role_at(*k).unwrap(), *k, v));
    }

    fn output(color: Color, role: Role, square: Square, moves: &HashMap<Option<Square>, u32>) {
        println!("{}{}:", Self::piece_char(color, role), square);
        moves.iter().for_each(|(to_square, count)| {
            let to = if let Some(s) = to_square { s.to_string() }
            else { "nowhere".to_string() };
            println!("    {}: {}", to, count);
        });
    }

    #[allow(clippy::non_ascii_literal)]
    fn piece_char(color: Color, role: Role) -> &'a str {
        if color == Color::White {
            match role {
                Role::King   => "♔",
                Role::Queen  => "♕",
                Role::Rook   => "♖",
                Role::Bishop => "♗",
                Role::Knight => "♘",
                Role::Pawn   => "♙",
            }
        }
        else {
            match role {
                Role::King   => "♚",
                Role::Queen  => "♛",
                Role::Rook   => "♜",
                Role::Bishop => "♝",
                Role::Knight => "♞",
                Role::Pawn   => "♟",
            }
        }
    }
}

struct PgnSearcher {
    // Permanent state:
    target: Chess,
    counts: HashMap<Square, HashMap<Option<Square>, u32>>,

    // Per-game state:
    counting: bool,
    position: Chess,
    pieces: HashSet<Square>,
    counted: bool,
    skip_game: bool,

    // Per-game state for error reporting:
    turn: bool,
    move_no: u32,
    white: String,
    black: String,
    date: String,
}

impl PgnSearcher {
    fn new(target: Chess) -> Self {
        let pieces = Self::pieces_for(&target);
        Self { target
             , counts: Self::initial_counts(&pieces)

             , counting: false
             , position: Chess::default()
             , pieces
             , counted: false
             , skip_game: false

             , turn: true
             , move_no: 0
             , white: "".to_string()
             , black: "".to_string()
             , date: "".to_string()
             }
    }

    fn pieces_for(target: &Chess) -> HashSet<Square> {
        let mut pieces = target.us();
        pieces.add(target.them());
        HashSet::from_iter(pieces)
    }

    fn initial_counts(pieces: &HashSet<Square>) -> HashMap<Square, HashMap<Option<Square>, u32>> {
        HashMap::from_iter(pieces.iter().map(|s| (*s, HashMap::default())))
    }

    fn observe(&mut self, m: &Move) {
        if self.counting {
            match m {
                Move::Normal { from, to, .. } => {
                    if self.pieces.contains(from) {
                        self.count(*from, Some(*to))
                    }

                    if self.pieces.contains(to) {
                        self.count(*to, Some(*to))
                    }
                },
                Move::EnPassant { from, to } => {
                    if self.pieces.contains(from) {
                        self.count(*from, Some(*to));
                    }

                    let captured = to.combine(*from);
                    if self.pieces.contains(&captured) {
                        self.count(captured, Some(captured));
                    }
                },
                Move::Castle { king, rook } => {
                    self.count(*king, Some(*rook));
                    /* XXX: We need to chain the two flips here, since
                     * .flip_diagonal() is a noop for squares on the a1-h8
                     * diagonal, causing long castle for white and short
                     * castle for black to be conflated with un-moved rook.
                     */
                    self.count(*rook, Some(rook.flip_horizontal().flip_vertical()));
                },
                Move::Put { .. } => panic!("Can't handle puts"),
            }
        }
    }

    fn count(&mut self, piece: Square, to: Option<Square>) {
        self.pieces.remove(&piece);
        Self::do_count(&mut self.counts, piece, to);
    }

    fn do_count(counts: &mut HashMap<Square, HashMap<Option<Square>, u32>>, piece: Square, to: Option<Square>) {
        let c = counts.get_mut(&piece).unwrap();
        c.entry(to).and_modify(|c| *c += 1).or_insert(1);
    }

    fn check_for_target(&mut self) {
        let t = &self.target;
        let p = &self.position;

        if !self.counting
        && t.turn() == p.turn()
        && t.castling_rights() == p.castling_rights()
        && t.board() == p.board() {
            self.counting = true;
            self.counted = true;
        }
        else if self.counting && self.pieces.is_empty() {
            self.counting = false;
        }
    }
}

impl Visitor for PgnSearcher {
    type Result = ();

    fn header(&mut self, key: &[u8], value: RawHeader) {
        let key = ::std::str::from_utf8(key).unwrap();
        let value = value.decode_utf8().unwrap().to_string();

        if key == "White" {
            self.white = value;
        }
        else if key == "Black" {
            self.black = value;
        }
        else if key == "Date" {
            self.date = value;
        }
        else if key == "FEN" {
            self.skip_game = true;
        }
    }

    fn end_headers(&mut self) -> Skip {
        Skip(self.skip_game)
    }

    /* We ignore variations for now (they cause self.position to go bananas
     * when the variation ends. */
    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.turn { self.move_no += 1 }

        if let Ok(m) = san_plus.san.to_move(&self.position) {
            self.check_for_target();
            self.observe(&m);
            self.position.play_unchecked(&m);
        }
        else {
            let move_follow = if self.turn { " " } else { " ..." };
            panic!("Failed to parse move {}.{}{}.\nIn {}-{} ({})",
                self.move_no, move_follow, san_plus,
                self.white, self.black, self.date);
        }
        self.turn = !self.turn;
    }

    fn end_game(&mut self) -> Self::Result {
        // Count unmoved pieces:
        if self.counted {
            let c = &mut self.counts;
            self.pieces.iter().for_each(|s| Self::do_count(c, *s, None));
        }

        // Reset per-game state for next game in reader:
        self.position = Chess::default();
        self.pieces = Self::pieces_for(&self.target);
        self.counting = false;
        self.counted = false;
        self.skip_game = false;

        self.turn = true;
        self.move_no = 0;
        self.white = "".to_string();
        self.black = "".to_string();
        self.date = "".to_string();
    }
}

fn main() {
    let doc = html! {
        html {
            head {
                script { (chessground()) }
                style { (style()) }
            }
            body {
                div.blue.merida {
                    div."cg-board-wrap"#board;
                }
            }
        }
    };
    println!("{}", doc.into_string());
    return;



    let matches = App::new("first-move")
        .arg(Arg::with_name("input")
             .required(true)
             .index(1))
        .arg(Arg::with_name("position")
             .required(true)
             .index(2))
        .get_matches();
    let mut search = Search::new(&matches.value_of("input").unwrap());
    search.run(&matches.value_of("position").unwrap());
}
