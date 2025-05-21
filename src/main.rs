#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use std::marker::ConstParamTy;


// 1. ヘルパー要素
// -----------------------------------------------------------------------------

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
    const Cell1: Cell, const Cell2: Cell, const Cell3: Cell, const Cell4: Cell, const Cell5: Cell, const Cell6: Cell, const Cell7: Cell, const Cell8: Cell, const Cell9: Cell,
    const Cell10: Cell, const Cell11: Cell, const Cell12: Cell, const Cell13: Cell, const Cell14: Cell, const Cell15: Cell, const Cell16: Cell, const Cell17: Cell, const Cell18: Cell,
    const Cell19: Cell, const Cell20: Cell, const Cell21: Cell, const Cell22: Cell, const Cell23: Cell, const Cell24: Cell, const Cell25: Cell, const Cell26: Cell, const Cell27: Cell,
    const Cell28: Cell, const Cell29: Cell, const Cell30: Cell, const Cell31: Cell, const Cell32: Cell, const Cell33: Cell, const Cell34: Cell, const Cell35: Cell, const Cell36: Cell,
    const Cell37: Cell, const Cell38: Cell, const Cell39: Cell, const Cell40: Cell, const Cell41: Cell, const Cell42: Cell, const Cell43: Cell, const Cell44: Cell, const Cell45: Cell,
    const Cell46: Cell, const Cell47: Cell, const Cell48: Cell, const Cell49: Cell, const Cell50: Cell, const Cell51: Cell, const Cell52: Cell, const Cell53: Cell, const Cell54: Cell,
    const Cell55: Cell, const Cell56: Cell, const Cell57: Cell, const Cell58: Cell, const Cell59: Cell, const Cell60: Cell, const Cell61: Cell, const Cell62: Cell, const Cell63: Cell,
    const Cell64: Cell, const Cell65: Cell, const Cell66: Cell, const Cell67: Cell, const Cell68: Cell, const Cell69: Cell, const Cell70: Cell, const Cell71: Cell, const Cell72: Cell,
    const Cell73: Cell, const Cell74: Cell, const Cell75: Cell, const Cell76: Cell, const Cell77: Cell, const Cell78: Cell, const Cell79: Cell, const Cell80: Cell, const Cell81: Cell,
>(
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
    Cell, Cell, Cell,  Cell, Cell, Cell,  Cell, Cell, Cell,
)
where
    // 行の判定
    Assert<{ is_unique(Cell1, Cell2, Cell3, Cell4, Cell5, Cell6, Cell7, Cell8, Cell9) }>: IsTrue,
    Assert<{ is_unique(Cell10, Cell11, Cell12, Cell13, Cell14, Cell15, Cell16, Cell17, Cell18) }>: IsTrue,
    Assert<{ is_unique(Cell19, Cell20, Cell21, Cell22, Cell23, Cell24, Cell25, Cell26, Cell27) }>: IsTrue,
    Assert<{ is_unique(Cell28, Cell29, Cell30, Cell31, Cell32, Cell33, Cell34, Cell35, Cell36) }>: IsTrue,
    Assert<{ is_unique(Cell37, Cell38, Cell39, Cell40, Cell41, Cell42, Cell43, Cell44, Cell45) }>: IsTrue,
    Assert<{ is_unique(Cell46, Cell47, Cell48, Cell49, Cell50, Cell51, Cell52, Cell53, Cell54) }>: IsTrue,
    Assert<{ is_unique(Cell55, Cell56, Cell57, Cell58, Cell59, Cell60, Cell61, Cell62, Cell63) }>: IsTrue,
    Assert<{ is_unique(Cell64, Cell65, Cell66, Cell67, Cell68, Cell69, Cell70, Cell71, Cell72) }>: IsTrue,
    Assert<{ is_unique(Cell73, Cell74, Cell75, Cell76, Cell77, Cell78, Cell79, Cell80, Cell81) }>: IsTrue,

    // 列の判定
    Assert<{ is_unique(Cell1, Cell10, Cell19, Cell28, Cell37, Cell46, Cell55, Cell64, Cell73) }>: IsTrue,
    Assert<{ is_unique(Cell2, Cell11, Cell20, Cell29, Cell38, Cell47, Cell56, Cell65, Cell74) }>: IsTrue,
    Assert<{ is_unique(Cell3, Cell12, Cell21, Cell30, Cell39, Cell48, Cell57, Cell66, Cell75) }>: IsTrue,
    Assert<{ is_unique(Cell4, Cell13, Cell22, Cell31, Cell40, Cell49, Cell58, Cell67, Cell76) }>: IsTrue,
    Assert<{ is_unique(Cell5, Cell14, Cell23, Cell32, Cell41, Cell50, Cell59, Cell68, Cell77) }>: IsTrue,
    Assert<{ is_unique(Cell6, Cell15, Cell24, Cell33, Cell42, Cell51, Cell60, Cell69, Cell78) }>: IsTrue,
    Assert<{ is_unique(Cell7, Cell16, Cell25, Cell34, Cell43, Cell52, Cell61, Cell70, Cell79) }>: IsTrue,
    Assert<{ is_unique(Cell8, Cell17, Cell26, Cell35, Cell44, Cell53, Cell62, Cell71, Cell80) }>: IsTrue,
    Assert<{ is_unique(Cell9, Cell18, Cell27, Cell36, Cell45, Cell54, Cell63, Cell72, Cell81) }>: IsTrue,

    // ブロックの判定
    Assert<{ is_unique(Cell1, Cell2, Cell3, Cell10, Cell11, Cell12, Cell19, Cell20, Cell21) }>: IsTrue,
    Assert<{ is_unique(Cell4, Cell5, Cell6, Cell13, Cell14, Cell15, Cell22, Cell23, Cell24) }>: IsTrue,
    Assert<{ is_unique(Cell7, Cell8, Cell9, Cell16, Cell17, Cell18, Cell25, Cell26, Cell27) }>: IsTrue,
    Assert<{ is_unique(Cell28, Cell29, Cell30, Cell37, Cell38, Cell39, Cell46, Cell47, Cell48) }>: IsTrue,
    Assert<{ is_unique(Cell31, Cell32, Cell33, Cell40, Cell41, Cell42, Cell49, Cell50, Cell51) }>: IsTrue,
    Assert<{ is_unique(Cell52, Cell53, Cell54, Cell61, Cell62, Cell63, Cell70, Cell71, Cell72) }>: IsTrue,
    Assert<{ is_unique(Cell55, Cell56, Cell57, Cell64, Cell65, Cell66, Cell73, Cell74, Cell75) }>: IsTrue,
    Assert<{ is_unique(Cell58, Cell59, Cell60, Cell67, Cell68, Cell69, Cell76, Cell77, Cell78) }>: IsTrue,
    Assert<{ is_unique(Cell61, Cell62, Cell63, Cell70, Cell71, Cell72, Cell79, Cell80, Cell81) }>: IsTrue,
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

    const NN: Cell = Cell::Empty;

    let sudoku = Sudoku::<
        C1, C3, C2,  NN, NN, NN,  NN, NN, NN, 
        NN, NN, C4,  NN, NN, NN,  C9, NN, NN, 
        NN, C5, NN,  NN, NN, NN,  NN, C1, NN, 
        
        NN, NN, NN,  C2, NN, NN,  NN, NN, NN, 
        NN, NN, C1,  NN, NN, NN,  NN, NN, NN, 
        NN, C6, NN,  C7, NN, C1,  C2, NN, C8, 
        
        NN, NN, NN,  NN, NN, NN,  C1, NN, NN, 
        NN, NN, NN,  NN, NN, C2,  NN, NN, NN, 
        C2, NN, NN,  NN, NN, NN,  NN, NN, NN, 
    >;
}