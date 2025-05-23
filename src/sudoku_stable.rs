struct _1;
struct _2;
struct _3;
struct _4;
struct _5;
struct _6;
struct _7;
struct _8;
struct _9;

struct __;

trait IsCell {}
impl IsCell for _1 {}
impl IsCell for _2 {}
impl IsCell for _3 {}
impl IsCell for _4 {}
impl IsCell for _5 {}
impl IsCell for _6 {}
impl IsCell for _7 {}
impl IsCell for _8 {}
impl IsCell for _9 {}
impl IsCell for __ {}


pub trait IsDiffType<T, U> {}
impl IsDiffType<_1, _2> for () {}
impl IsDiffType<_1, _3> for () {}
impl IsDiffType<_1, _4> for () {}
impl IsDiffType<_1, _5> for () {}
impl IsDiffType<_1, _6> for () {}
impl IsDiffType<_1, _7> for () {}
impl IsDiffType<_1, _8> for () {}
impl IsDiffType<_1, _9> for () {}

impl IsDiffType<_2, _1> for () {}
impl IsDiffType<_2, _3> for () {}
impl IsDiffType<_2, _4> for () {}
impl IsDiffType<_2, _5> for () {}
impl IsDiffType<_2, _6> for () {}
impl IsDiffType<_2, _7> for () {}
impl IsDiffType<_2, _8> for () {}
impl IsDiffType<_2, _9> for () {}

impl IsDiffType<_3, _1> for () {}
impl IsDiffType<_3, _2> for () {}
impl IsDiffType<_3, _4> for () {}
impl IsDiffType<_3, _5> for () {}
impl IsDiffType<_3, _6> for () {}
impl IsDiffType<_3, _7> for () {}
impl IsDiffType<_3, _8> for () {}
impl IsDiffType<_3, _9> for () {}

impl IsDiffType<_4, _1> for () {}
impl IsDiffType<_4, _2> for () {}
impl IsDiffType<_4, _3> for () {}
impl IsDiffType<_4, _5> for () {}
impl IsDiffType<_4, _6> for () {}
impl IsDiffType<_4, _7> for () {}
impl IsDiffType<_4, _8> for () {}
impl IsDiffType<_4, _9> for () {}

impl IsDiffType<_5, _1> for () {}
impl IsDiffType<_5, _2> for () {}
impl IsDiffType<_5, _3> for () {}
impl IsDiffType<_5, _4> for () {}
impl IsDiffType<_5, _6> for () {}
impl IsDiffType<_5, _7> for () {}
impl IsDiffType<_5, _8> for () {}
impl IsDiffType<_5, _9> for () {}

impl IsDiffType<_6, _1> for () {}
impl IsDiffType<_6, _2> for () {}
impl IsDiffType<_6, _3> for () {}
impl IsDiffType<_6, _4> for () {}
impl IsDiffType<_6, _5> for () {}
impl IsDiffType<_6, _7> for () {}
impl IsDiffType<_6, _8> for () {}
impl IsDiffType<_6, _9> for () {}

impl IsDiffType<_7, _1> for () {}
impl IsDiffType<_7, _2> for () {}
impl IsDiffType<_7, _3> for () {}
impl IsDiffType<_7, _4> for () {}
impl IsDiffType<_7, _5> for () {}
impl IsDiffType<_7, _6> for () {}
impl IsDiffType<_7, _8> for () {}
impl IsDiffType<_7, _9> for () {}

impl IsDiffType<_8, _1> for () {}
impl IsDiffType<_8, _2> for () {}
impl IsDiffType<_8, _3> for () {}
impl IsDiffType<_8, _4> for () {}
impl IsDiffType<_8, _5> for () {}
impl IsDiffType<_8, _6> for () {}
impl IsDiffType<_8, _7> for () {}
impl IsDiffType<_8, _9> for () {}

impl IsDiffType<_9, _1> for () {}
impl IsDiffType<_9, _2> for () {}
impl IsDiffType<_9, _3> for () {}
impl IsDiffType<_9, _4> for () {}
impl IsDiffType<_9, _5> for () {}
impl IsDiffType<_9, _6> for () {}
impl IsDiffType<_9, _7> for () {}
impl IsDiffType<_9, _8> for () {}

impl IsDiffType<_1, __> for () {}
impl IsDiffType<_2, __> for () {}
impl IsDiffType<_3, __> for () {}
impl IsDiffType<_4, __> for () {}
impl IsDiffType<_5, __> for () {}
impl IsDiffType<_6, __> for () {}
impl IsDiffType<_7, __> for () {}
impl IsDiffType<_8, __> for () {}
impl IsDiffType<_9, __> for () {}
impl IsDiffType<__, __> for () {}
impl IsDiffType<__, _1> for () {}
impl IsDiffType<__, _2> for () {}
impl IsDiffType<__, _3> for () {}
impl IsDiffType<__, _4> for () {}
impl IsDiffType<__, _5> for () {}
impl IsDiffType<__, _6> for () {}
impl IsDiffType<__, _7> for () {}
impl IsDiffType<__, _8> for () {}
impl IsDiffType<__, _9> for () {}

pub trait AreDiffTypeParams<T1, T2, T3, T4, T5, T6, T7, T8, T9> {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> AreDiffTypeParams<T1, T2, T3, T4, T5, T6, T7, T8, T9> for ()
where
    (): IsDiffType<T1, T2> + IsDiffType<T1, T3> + IsDiffType<T1, T4> + IsDiffType<T1, T5> + IsDiffType<T1, T6> + IsDiffType<T1, T7> + IsDiffType<T1, T8> + IsDiffType<T1, T9>,
    (): IsDiffType<T2, T3> + IsDiffType<T2, T4> + IsDiffType<T2, T5> + IsDiffType<T2, T6> + IsDiffType<T2, T7> + IsDiffType<T2, T8> + IsDiffType<T2, T9>,
    (): IsDiffType<T3, T4> + IsDiffType<T3, T5> + IsDiffType<T3, T6> + IsDiffType<T3, T7> + IsDiffType<T3, T8> + IsDiffType<T3, T9>,
    (): IsDiffType<T4, T5> + IsDiffType<T4, T6> + IsDiffType<T4, T7> + IsDiffType<T4, T8> + IsDiffType<T4, T9>,
    (): IsDiffType<T5, T6> + IsDiffType<T5, T7> + IsDiffType<T5, T8> + IsDiffType<T5, T9>,
    (): IsDiffType<T6, T7> + IsDiffType<T6, T8> + IsDiffType<T6, T9>,
    (): IsDiffType<T7, T8> + IsDiffType<T7, T9>,
    (): IsDiffType<T8, T9>,
{}


struct Sudoku<
    X11, X12, X13, X14, X15, X16, X17, X18, X19,
    X21, X22, X23, X24, X25, X26, X27, X28, X29,
    X31, X32, X33, X34, X35, X36, X37, X38, X39,
    X41, X42, X43, X44, X45, X46, X47, X48, X49,
    X51, X52, X53, X54, X55, X56, X57, X58, X59,
    X61, X62, X63, X64, X65, X66, X67, X68, X69,
    X71, X72, X73, X74, X75, X76, X77, X78, X79,
    X81, X82, X83, X84, X85, X86, X87, X88, X89,
    X91, X92, X93, X94, X95, X96, X97, X98, X99
>
(
    X11, X12, X13, X14, X15, X16, X17, X18, X19,
    X21, X22, X23, X24, X25, X26, X27, X28, X29,
    X31, X32, X33, X34, X35, X36, X37, X38, X39,
    X41, X42, X43, X44, X45, X46, X47, X48, X49,
    X51, X52, X53, X54, X55, X56, X57, X58, X59,
    X61, X62, X63, X64, X65, X66, X67, X68, X69,
    X71, X72, X73, X74, X75, X76, X77, X78, X79,
    X81, X82, X83, X84, X85, X86, X87, X88, X89,
    X91, X92, X93, X94, X95, X96, X97, X98, X99
)
where
    X11: IsCell, X12: IsCell, X13: IsCell, X14: IsCell, X15: IsCell, X16: IsCell, X17: IsCell, X18: IsCell, X19: IsCell,
    X21: IsCell, X22: IsCell, X23: IsCell, X24: IsCell, X25: IsCell, X26: IsCell, X27: IsCell, X28: IsCell, X29: IsCell,
    X31: IsCell, X32: IsCell, X33: IsCell, X34: IsCell, X35: IsCell, X36: IsCell, X37: IsCell, X38: IsCell, X39: IsCell,
    X41: IsCell, X42: IsCell, X43: IsCell, X44: IsCell, X45: IsCell, X46: IsCell, X47: IsCell, X48: IsCell, X49: IsCell,
    X51: IsCell, X52: IsCell, X53: IsCell, X54: IsCell, X55: IsCell, X56: IsCell, X57: IsCell, X58: IsCell, X59: IsCell,
    X61: IsCell, X62: IsCell, X63: IsCell, X64: IsCell, X65: IsCell, X66: IsCell, X67: IsCell, X68: IsCell, X69: IsCell,
    X71: IsCell, X72: IsCell, X73: IsCell, X74: IsCell, X75: IsCell, X76: IsCell, X77: IsCell, X78: IsCell, X79: IsCell,
    X81: IsCell, X82: IsCell, X83: IsCell, X84: IsCell, X85: IsCell, X86: IsCell, X87: IsCell, X88: IsCell, X89: IsCell,
    X91: IsCell, X92: IsCell, X93: IsCell, X94: IsCell, X95: IsCell, X96: IsCell, X97: IsCell, X98: IsCell, X99: IsCell,
    
    // 行のチェック
    (): AreDiffTypeParams<X11, X12, X13, X14, X15, X16, X17, X18, X19>,
    (): AreDiffTypeParams<X21, X22, X23, X24, X25, X26, X27, X28, X29>,
    (): AreDiffTypeParams<X31, X32, X33, X34, X35, X36, X37, X38, X39>,
    (): AreDiffTypeParams<X41, X42, X43, X44, X45, X46, X47, X48, X49>,
    (): AreDiffTypeParams<X51, X52, X53, X54, X55, X56, X57, X58, X59>,
    (): AreDiffTypeParams<X61, X62, X63, X64, X65, X66, X67, X68, X69>,
    (): AreDiffTypeParams<X71, X72, X73, X74, X75, X76, X77, X78, X79>,
    (): AreDiffTypeParams<X81, X82, X83, X84, X85, X86, X87, X88, X89>,
    (): AreDiffTypeParams<X91, X92, X93, X94, X95, X96, X97, X98, X99>,

    // 列のチェック
    (): AreDiffTypeParams<X11, X21, X31, X41, X51, X61, X71, X81, X91>,
    (): AreDiffTypeParams<X12, X22, X32, X42, X52, X62, X72, X82, X92>,
    (): AreDiffTypeParams<X13, X23, X33, X43, X53, X63, X73, X83, X93>,
    (): AreDiffTypeParams<X14, X24, X34, X44, X54, X64, X74, X84, X94>,
    (): AreDiffTypeParams<X15, X25, X35, X45, X55, X65, X75, X85, X95>,
    (): AreDiffTypeParams<X16, X26, X36, X46, X56, X66, X76, X86, X96>,
    (): AreDiffTypeParams<X17, X27, X37, X47, X57, X67, X77, X87, X97>,
    (): AreDiffTypeParams<X18, X28, X38, X48, X58, X68, X78, X88, X98>,
    (): AreDiffTypeParams<X19, X29, X39, X49, X59, X69, X79, X89, X99>,
    
    // ブロックのチェック
    (): AreDiffTypeParams<X11, X12, X13, X21, X22, X23, X31, X32, X33>
    + AreDiffTypeParams<X14, X15, X16, X24, X25, X26, X34, X35, X36>
    + AreDiffTypeParams<X17, X18, X19, X27, X28, X29, X37, X38, X39>
    + AreDiffTypeParams<X41, X42, X43, X51, X52, X53, X61, X62, X63>
    + AreDiffTypeParams<X44, X45, X46, X54, X55, X56, X64, X65, X66>
    + AreDiffTypeParams<X47, X48, X49, X57, X58, X59, X67, X68, X69>
    + AreDiffTypeParams<X71, X72, X73, X81, X82, X83, X91, X92, X93>
    + AreDiffTypeParams<X74, X75, X76, X84, X85, X86, X94, X95, X96>
    + AreDiffTypeParams<X77, X78, X79, X87, X88, X89, X97, X98, X99>;


fn main() {
    let sudoku = Sudoku (
        _5, _3, _4,  _6, _7, _8,  _9, _1, _2,
        _6, _7, _2,  _1, _9, _5,  _3, _4, _8,
        _1, _9, _8,  _3, _4, _2,  _5, _6, _7,

        _8, _5, _9,  _7, _6, _1,  _4, _2, _3,
        _4, _2, _6,  _8, _5, _3,  _7, _9, _1,
        _7, _1, _3,  _9, _2, _4,  _8, _5, _6,

        _9, _6, _1,  _5, _3, _7,  _2, _8, _4,
        _2, _8, _7,  _4, _1, _9,  _6, _3, _5,
        _3, _4, _5,  _2, _8, _6,  _1, _7, _9,
    );
}