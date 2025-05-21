#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use std::marker::ConstParamTy;

struct Assert<const COND: bool>;
trait IsTrue {}
impl IsTrue for Assert<true> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
enum Num {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
enum Cell {
    Empty,
    Filled(Num),
}

const fn not_eq(a: Cell, b: Cell) -> bool {
    match (a, b) {
        (Cell::Filled(x), Cell::Filled(y)) => x as u8 != y as u8,
        (Cell::Empty, Cell::Empty) => true,
        (Cell::Empty, Cell::Filled(_)) | (Cell::Filled(_), Cell::Empty) => true,
    }
}

const fn is_unique(
    C1: Cell, C2: Cell, C3: Cell, C4: Cell, C5: Cell, C6: Cell, C7: Cell, C8: Cell, C9: Cell
) -> bool {
    not_eq(C1, C2) && not_eq(C1, C3) && not_eq(C1, C4) && not_eq(C1, C5) && not_eq(C1, C6) && not_eq(C1, C7) && not_eq(C1, C8) && not_eq(C1, C9)
    && not_eq(C2, C3) && not_eq(C2, C4) && not_eq(C2, C5) && not_eq(C2, C6) && not_eq(C2, C7) && not_eq(C2, C8) && not_eq(C2, C9)
    && not_eq(C3, C4) && not_eq(C3, C5) && not_eq(C3, C6) && not_eq(C3, C7) && not_eq(C3, C8) && not_eq(C3, C9)
    && not_eq(C4, C5) && not_eq(C4, C6) && not_eq(C4, C7) && not_eq(C4, C8) && not_eq(C4, C9)
    && not_eq(C5, C6) && not_eq(C5, C7) && not_eq(C5, C8) && not_eq(C5, C9)
    && not_eq(C6, C7) && not_eq(C6, C8) && not_eq(C6, C9)
    && not_eq(C7, C8) && not_eq(C7, C9)
    && not_eq(C8, C9)
}

struct Sudoku<
    //const Cells: [[Cell; 9]; 9]
    const X11: Cell, const X12: Cell, const X13: Cell, const X14: Cell, const X15: Cell, const X16: Cell, const X17: Cell, const X18: Cell, const X19: Cell,
    const X21: Cell, const X22: Cell, const X23: Cell, const X24: Cell, const X25: Cell, const X26: Cell, const X27: Cell, const X28: Cell, const X29: Cell,
    const X31: Cell, const X32: Cell, const X33: Cell, const X34: Cell, const X35: Cell, const X36: Cell, const X37: Cell, const X38: Cell, const X39: Cell,
    const X41: Cell, const X42: Cell, const X43: Cell, const X44: Cell, const X45: Cell, const X46: Cell, const X47: Cell, const X48: Cell, const X49: Cell,
    const X51: Cell, const X52: Cell, const X53: Cell, const X54: Cell, const X55: Cell, const X56: Cell, const X57: Cell, const X58: Cell, const X59: Cell,
    const X61: Cell, const X62: Cell, const X63: Cell, const X64: Cell, const X65: Cell, const X66: Cell, const X67: Cell, const X68: Cell, const X69: Cell,
    const X71: Cell, const X72: Cell, const X73: Cell, const X74: Cell, const X75: Cell, const X76: Cell, const X77: Cell, const X78: Cell, const X79: Cell,
    const X81: Cell, const X82: Cell, const X83: Cell, const X84: Cell, const X85: Cell, const X86: Cell, const X87: Cell, const X88: Cell, const X89: Cell,
    const X91: Cell, const X92: Cell, const X93: Cell, const X94: Cell, const X95: Cell, const X96: Cell, const X97: Cell, const X98: Cell, const X99: Cell
>([[Cell; 9]; 9])
where
    // 行の判定
    Assert<{ is_unique(X11, X12, X13, X14, X15, X16, X17, X18, X19) }>: IsTrue,
    Assert<{ is_unique(X21, X22, X23, X24, X25, X26, X27, X28, X29) }>: IsTrue,
    Assert<{ is_unique(X31, X32, X33, X34, X35, X36, X37, X38, X39) }>: IsTrue,
    Assert<{ is_unique(X41, X42, X43, X44, X45, X46, X47, X48, X49) }>: IsTrue,
    Assert<{ is_unique(X51, X52, X53, X54, X55, X56, X57, X58, X59) }>: IsTrue,
    Assert<{ is_unique(X61, X62, X63, X64, X65, X66, X67, X68, X69) }>: IsTrue,
    Assert<{ is_unique(X71, X72, X73, X74, X75, X76, X77, X78, X79) }>: IsTrue,
    Assert<{ is_unique(X81, X82, X83, X84, X85, X86, X87, X88, X89) }>: IsTrue,
    Assert<{ is_unique(X91, X92, X93, X94, X95, X96, X97, X98, X99) }>: IsTrue,

    // 列の判定
    Assert<{ is_unique(X11, X21, X31, X41, X51, X61, X71, X81, X91) }>: IsTrue,
    Assert<{ is_unique(X12, X22, X32, X42, X52, X62, X72, X82, X92) }>: IsTrue,
    Assert<{ is_unique(X13, X23, X33, X43, X53, X63, X73, X83, X93) }>: IsTrue,
    Assert<{ is_unique(X14, X24, X34, X44, X54, X64, X74, X84, X94) }>: IsTrue,
    Assert<{ is_unique(X15, X25, X35, X45, X55, X65, X75, X85, X95) }>: IsTrue,
    Assert<{ is_unique(X16, X26, X36, X46, X56, X66, X76, X86, X96) }>: IsTrue,
    Assert<{ is_unique(X17, X27, X37, X47, X57, X67, X77, X87, X97) }>: IsTrue,
    Assert<{ is_unique(X18, X28, X38, X48, X58, X68, X78, X88, X98) }>: IsTrue,
    Assert<{ is_unique(X19, X29, X39, X49, X59, X69, X79, X89, X99) }>: IsTrue,

    // ブロックの判定
    Assert<{ is_unique(X11, X12, X13, X21, X22, X23, X31, X32, X33) }>: IsTrue,
    Assert<{ is_unique(X14, X15, X16, X24, X25, X26, X34, X35, X36) }>: IsTrue,
    Assert<{ is_unique(X17, X18, X19, X27, X28, X29, X37, X38, X39) }>: IsTrue,
    
    Assert<{ is_unique(X41, X42, X43, X51, X52, X53, X61, X62, X63) }>: IsTrue,
    Assert<{ is_unique(X44, X45, X46, X54, X55, X56, X64, X65, X66) }>: IsTrue,
    Assert<{ is_unique(X47, X48, X49, X57, X58, X59, X67, X68, X69) }>: IsTrue,
    
    Assert<{ is_unique(X71, X72, X73, X81, X82, X83, X91, X92, X93) }>: IsTrue,
    Assert<{ is_unique(X74, X75, X76, X84, X85, X86, X94, X95, X96) }>: IsTrue,
    Assert<{ is_unique(X77, X78, X79, X87, X88, X89, X97, X98, X99) }>: IsTrue
;

fn main() {
    const C1: Cell = Cell::Filled(Num::One);
    const C2: Cell = Cell::Filled(Num::Two);
    const C3: Cell = Cell::Filled(Num::Three);
    const C4: Cell = Cell::Filled(Num::Four);
    const C5: Cell = Cell::Filled(Num::Five);
    const C6: Cell = Cell::Filled(Num::Six);
    const C7: Cell = Cell::Filled(Num::Seven);
    const C8: Cell = Cell::Filled(Num::Eight);
    const C9: Cell = Cell::Filled(Num::Nine);

    const __: Cell = Cell::Empty;

    let sudoku = Sudoku::<
        C1, C3, C2,  __, __, __,  __, __, __,
        __, __, C4,  __, __, __,  C9, __, __,
        __, C5, __,  __, __, __,  __, C1, __,

        __, __, __,  C2, __, __,  __, __, __,
        __, __, C1,  __, __, __,  __, __, __,
        __, C6, __,  C7, __, C1,  C2, __, C8,

        __, __, __,  __, __, __,  C1, __, __,
        __, __, __,  __, __, C2,  __, __, __,
        C2, __, __,  __, __, __,  __, __, __,
    >;
}