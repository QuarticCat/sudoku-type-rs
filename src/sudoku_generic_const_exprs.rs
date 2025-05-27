#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
enum Num {
    _1 = (1 << 0),
    _2 = (1 << 1),
    _3 = (1 << 2),
    _4 = (1 << 3),
    _5 = (1 << 4),
    _6 = (1 << 5),
    _7 = (1 << 6),
    _8 = (1 << 7),
    _9 = (1 << 8),
    __ = (1 << 9),
}
use Num::*;

macro_rules! const_for {
    ($i:ident in $range:expr => $body:stmt) => {{
        let mut $i = $range.start;
        while $i < $range.end {
            $body
            $i += 1;
        }
    }};
}

const fn check_sudoku(array: [[Num; 9]; 9]) {
    const_for!(i in 0..9 => {
        let (mut 横行, mut 縦列, mut ブロック) = (0, 0, 0);
        const_for!(j in 0..9 => {
            横行 |= array[i][j] as i32;
            縦列 |= array[j][i] as i32;
            ブロック |= array[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3] as i32;
        });
        if 横行 != 0b111111111 || 縦列 != 0b111111111 || ブロック != 0b111111111 {
            panic!("invalid");
        }
    });
}

fn main() {
    #[rustfmt::skip]
    const _: () = check_sudoku([
        [_5, _3, _4,  _6, _7, _8,  _9, _1, _2],
        [_6, _7, _2,  _1, _9, _5,  _3, _4, _8],
        [_1, _9, _8,  _3, _4, _2,  _5, _6, _7],

        [_8, _5, _9,  _7, _6, _1,  _4, _2, _3],
        [_4, _2, _6,  _8, _5, _3,  _7, _9, _1],
        [_7, _1, _3,  _9, _2, _4,  _8, _5, _6],

        [_9, _6, _1,  _5, _3, _7,  _2, _8, _4],
        [_2, _8, _7,  _4, _1, _9,  _6, _3, _5],
        [_3, _4, _5,  _2, _8, _6,  _1, _7, _9],
    ]);
}
