include!(concat!(env!("OUT_DIR"), "/gen.rs"));

fn urlenc<'a>(s: &'a str) -> String {
    s.replace("#", "%23")
     .replace("<", "%3C")
     .replace(">", "%3E")
     .replace("?", "%3F")
     .replace("\n", "%0A")
}

pub fn chessground<'a>() -> &'a str { CHESSGROUND }
/* TODO: Style tables so that it's easy to connect numbers to pieces.
 * - Add borders between cells in outer table, and get text in "from" cell to align with top of cell
 */
pub fn style<'a>() -> String { format!(r#"
table.stats {{ border-collapse: collapse; }}
table.stats td {{
    border: 1px solid black;
    vertical-align: top;
}}
table.counts td {{
    border-width: 0;
}}
.cg-board-wrap {{
  width: 320px;
  height: 320px;
  position: relative;
  display: block;
  margin: 20px;
}}
.cg-board {{
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  line-height: 0;
  background-size: cover;
  cursor: pointer;
}}
.cg-board square {{
  position: absolute;
  top: 0;
  left: 0;
  width: 12.5%;
  height: 12.5%;
  pointer-events: none;
}}
.cg-board square.move-dest {{
  background: radial-gradient(rgba(20, 85, 30, 0.5) 22%, #208530 0, rgba(0, 0, 0, 0.3) 0, rgba(0, 0, 0, 0) 0);
  pointer-events: auto;
}}
.cg-board square.premove-dest {{
  background: radial-gradient(rgba(20, 30, 85, 0.5) 22%, #203085 0, rgba(0, 0, 0, 0.3) 0, rgba(0, 0, 0, 0) 0);
}}
.cg-board square.oc.move-dest {{
  background: radial-gradient(transparent 0%, transparent 80%, rgba(20, 85, 0, 0.3) 80%);
}}
.cg-board square.oc.premove-dest {{
  background: radial-gradient(transparent 0%, transparent 80%, rgba(20, 30, 85, 0.2) 80%);
}}
.cg-board square.move-dest:hover {{
  background: rgba(20, 85, 30, 0.3);
}}
.cg-board square.premove-dest:hover {{
  background: rgba(20, 30, 85, 0.2);
}}
.cg-board square.last-move {{
  will-change: transform;
  background-color: rgba(155, 199, 0, 0.41);
}}
.cg-board square.selected {{
  background-color: rgba(20, 85, 30, 0.5);
}}
.cg-board square.check {{
  background: radial-gradient(ellipse at center, rgba(255, 0, 0, 1) 0%, rgba(231, 0, 0, 1) 25%, rgba(169, 0, 0, 0) 89%, rgba(158, 0, 0, 0) 100%);
}}
.cg-board square.current-premove {{
  background-color: rgba(20, 30, 85, 0.5);
}}
.cg-board-wrap piece {{
  position: absolute;
  top: 0;
  left: 0;
  width: 12.5%;
  height: 12.5%;
  background-size: cover;
  z-index: 2;
  will-change: transform;
  pointer-events: none;
}}
.cg-board piece.dragging {{
  cursor: move;
  z-index: 9;
}}
.cg-board piece.anim {{
  z-index: 8;
}}
.cg-board piece.fading {{
  z-index: 1;
  opacity: 0.5;
}}
.cg-board-wrap square.move-dest:hover {{
  background-color: rgba(20, 85, 30, 0.3);
}}
.cg-board-wrap piece.ghost {{
  opacity: 0.3;
}}
.cg-board-wrap svg {{
  overflow: hidden;
  position: relative;
  top: 0px;
  left: 0px;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 2;
  opacity: 0.6;
}}
.cg-board-wrap svg image {{
  opacity: 0.5;
}}
.cg-board-wrap coords {{
  position: absolute;
  display: flex;
  pointer-events: none;
  opacity: 0.8;
  font-size: 9px;
}}
.cg-board-wrap coords.ranks {{
  right: -15px;
  top: 0;
  flex-flow: column-reverse;
  height: 100%;
  width: 12px;
}}
.cg-board-wrap coords.ranks.black {{
  flex-flow: column;
}}
.cg-board-wrap coords.files {{
  bottom: -16px;
  left: 0;
  flex-flow: row;
  width: 100%;
  height: 16px;
  text-transform: uppercase;
  text-align: center;
}}
.cg-board-wrap coords.files.black {{
  flex-flow: row-reverse;
}}
.cg-board-wrap coords coord {{
  flex: 1 1 auto;
}}
.cg-board-wrap coords.ranks coord {{
  transform: translateY(39%);
}}

/*
 * Board
 */
.blue .cg-board-wrap {{
  background-image: url('data:image/svg+xml,{board}');
}}

.merida .cg-board-wrap piece.pawn.white {{
  background-image: url('data:image/svg+xml,{wp}');
}}
.merida .cg-board-wrap piece.bishop.white {{
  background-image: url('data:image/svg+xml,{wb}');
}}
.merida .cg-board-wrap piece.knight.white {{
  background-image: url('data:image/svg+xml,{wn}');
}}
.merida .cg-board-wrap piece.rook.white {{
  background-image: url('data:image/svg+xml,{wr}');
}}
.merida .cg-board-wrap piece.queen.white {{
  background-image: url('data:image/svg+xml,{wq}');
}}
.merida .cg-board-wrap piece.king.white {{
  background-image: url('data:image/svg+xml,{wk}');
}}
.merida .cg-board-wrap piece.pawn.black {{
  background-image: url('data:image/svg+xml,{bp}');
}}
.merida .cg-board-wrap piece.bishop.black {{
  background-image: url('data:image/svg+xml,{bb}');
}}
.merida .cg-board-wrap piece.knight.black {{
  background-image: url('data:image/svg+xml,{bn}');
}}
.merida .cg-board-wrap piece.rook.black {{
  background-image: url('data:image/svg+xml,{br}');
}}
.merida .cg-board-wrap piece.queen.black {{
  background-image: url('data:image/svg+xml,{bq}');
}}
.merida .cg-board-wrap piece.king.black {{
  background-image: url('data:image/svg+xml,{bk}');
}}
"#,
    board=urlenc(BOARD),
    wp=urlenc(WP),
    wb=urlenc(WB),
    wn=urlenc(WN),
    wr=urlenc(WR),
    wq=urlenc(WQ),
    wk=urlenc(WK),
    bp=urlenc(BP),
    bb=urlenc(BB),
    bn=urlenc(BN),
    br=urlenc(BR),
    bq=urlenc(BQ),
    bk=urlenc(BK)) }
