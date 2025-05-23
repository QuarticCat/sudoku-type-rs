#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

type cell_type = u8;

struct Assert<const COND: bool>;

trait IsTrue {}
impl IsTrue for Assert<true> {}

/*
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
enum SudokuCell {
    Filled(Num),
    Empty,
}

const fn not_eq(a: SudokuCell, b: SudokuCell) -> bool {
    match (a, b) {
        (SudokuCell::Filled(x), SudokuCell::Filled(y)) => x as u8 != y as u8,
        (SudokuCell::Empty, SudokuCell::Empty) => true,
        (SudokuCell::Empty, SudokuCell::Filled(_)) | (SudokuCell::Filled(_), SudokuCell::Empty) => true,
    }
}
*/

pub struct Sudoku<const Array: [[cell_type; 9]; 9]>
where Assert<{ sudoku_is_valid(Array) }>: IsTrue;

const fn are_diff_type_params(x1: cell_type, x2: cell_type, x3: cell_type, x4: cell_type, x5: cell_type, x6: cell_type, x7: cell_type, x8: cell_type, x9: cell_type) -> bool { // Enumなどを使う場合は、その型用にmatchなどで不等を示すconst関数を実装する
    x1 != x2 && x1 != x3 && x1 != x4 && x1 != x5 && x1 != x6 && x1 != x7 && x1 != x8 && x1 != x9
    && x2 != x3 && x2 != x4 && x2 != x5 && x2 != x6 && x2 != x7 && x2 != x8 && x2 != x9
    && x3 != x4 && x3 != x5 && x3 != x6 && x3 != x7 && x3 != x8 && x3 != x9
    && x4 != x5 && x4 != x6 && x4 != x7 && x4 != x8 && x4 != x9
    && x5 != x6 && x5 != x7 && x5 != x8 && x5 != x9
    && x6 != x7 && x6 != x8 && x6 != x9
    && x7 != x8 && x7 != x9
    && x8 != x9
}

const fn sudoku_is_valid(array: [[cell_type; 9]; 9]) -> bool {
    ( // 横行
        are_diff_type_params(array[0][0], array[0][1], array[0][2], array[0][3], array[0][4], array[0][5], array[0][6], array[0][7], array[0][8])
        && are_diff_type_params(array[1][0], array[1][1], array[1][2], array[1][3], array[1][4], array[1][5], array[1][6], array[1][7], array[1][8])
        && are_diff_type_params(array[2][0], array[2][1], array[2][2], array[2][3], array[2][4], array[2][5], array[2][6], array[2][7], array[2][8])
        && are_diff_type_params(array[3][0], array[3][1], array[3][2], array[3][3], array[3][4], array[3][5], array[3][6], array[3][7], array[3][8])
        && are_diff_type_params(array[4][0], array[4][1], array[4][2], array[4][3], array[4][4], array[4][5], array[4][6], array[4][7], array[4][8])
        && are_diff_type_params(array[5][0], array[5][1], array[5][2], array[5][3], array[5][4], array[5][5], array[5][6], array[5][7], array[5][8])
        && are_diff_type_params(array[6][0], array[6][1], array[6][2], array[6][3], array[6][4], array[6][5], array[6][6], array[6][7], array[6][8])
        && are_diff_type_params(array[7][0], array[7][1], array[7][2], array[7][3], array[7][4], array[7][5], array[7][6], array[7][7], array[7][8])
        && are_diff_type_params(array[8][0], array[8][1], array[8][2], array[8][3], array[8][4], array[8][5], array[8][6], array[8][7], array[8][8])
    ) &&
    ( // 縦列
        are_diff_type_params(array[0][0], array[1][0], array[2][0], array[3][0], array[4][0], array[5][0], array[6][0], array[7][0], array[8][0])
        && are_diff_type_params(array[0][1], array[1][1], array[2][1], array[3][1], array[4][1], array[5][1], array[6][1], array[7][1], array[8][1])
        && are_diff_type_params(array[0][2], array[1][2], array[2][2], array[3][2], array[4][2], array[5][2], array[6][2], array[7][2], array[8][2])
        && are_diff_type_params(array[0][3], array[1][3], array[2][3], array[3][3], array[4][3], array[5][3], array[6][3], array[7][3], array[8][3])
        && are_diff_type_params(array[0][4], array[1][4], array[2][4], array[3][4], array[4][4], array[5][4], array[6][4], array[7][4], array[8][4])
        && are_diff_type_params(array[0][5], array[1][5], array[2][5], array[3][5], array[4][5], array[5][5], array[6][5], array[7][5], array[8][5])
        && are_diff_type_params(array[0][6], array[1][6], array[2][6], array[3][6], array[4][6], array[5][6], array[6][6], array[7][6], array[8][6])
        && are_diff_type_params(array[0][7], array[1][7], array[2][7], array[3][7], array[4][7], array[5][7], array[6][7], array[7][7], array[8][7])
        && are_diff_type_params(array[0][8], array[1][8], array[2][8], array[3][8], array[4][8], array[5][8], array[6][8], array[7][8], array[8][8])
    ) &&
    ( // ブロック
        are_diff_type_params(array[0][0], array[0][1], array[0][2], array[1][0], array[1][1], array[1][2], array[2][0], array[2][1], array[2][2])
        && are_diff_type_params(array[0][3], array[0][4], array[0][5], array[1][3], array[1][4], array[1][5], array[2][3], array[2][4], array[2][5])
        && are_diff_type_params(array[0][6], array[0][7], array[0][8], array[1][6], array[1][7], array[1][8], array[2][6], array[2][7], array[2][8])
        && are_diff_type_params(array[3][0], array[3][1], array[3][2], array[4][0], array[4][1], array[4][2], array[5][0], array[5][1], array[5][2])
        && are_diff_type_params(array[3][3], array[3][4], array[3][5], array[4][3], array[4][4], array[4][5], array[5][3], array[5][4], array[5][5])
        && are_diff_type_params(array[3][6], array[3][7], array[3][8], array[4][6], array[4][7], array[4][8], array[5][6], array[5][7], array[5][8])
        && are_diff_type_params(array[6][0], array[6][1], array[6][2], array[7][0], array[7][1], array[7][2], array[8][0], array[8][1], array[8][2])
        && are_diff_type_params(array[6][3], array[6][4], array[6][5], array[7][3], array[7][4], array[7][5], array[8][3], array[8][4], array[8][5])
        && are_diff_type_params(array[6][6], array[6][7], array[6][8], array[7][6], array[7][7], array[7][8], array[8][6], array[8][7], array[8][8])
    )
}


fn main() {
    let sudoku = Sudoku::<{[
        [5, 3, 4,  6, 7, 8,  9, 1, 2],
        [6, 7, 2,  1, 9, 5,  3, 4, 8],
        [1, 9, 8,  3, 4, 2,  5, 6, 7],

        [8, 5, 9,  7, 6, 1,  4, 2, 3],
        [4, 2, 6,  8, 5, 3,  7, 9, 1],
        [7, 1, 3,  9, 2, 4,  8, 5, 6],

        [9, 6, 1,  5, 3, 7,  2, 8, 4],
        [2, 8, 7,  4, 1, 9,  6, 3, 5],
        [3, 4, 5,  2, 8, 6,  1, 7, 9],
    ]}>;
}