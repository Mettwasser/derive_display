use derive_display::Display;

#[derive(Display)]
pub enum MyEnum<'a> {
    #[display("A: {}", x)]
    A { x: i32 },

    #[display("B: {}", _0)]
    B(&'a str),

    #[display("C: Unit")]
    C,
}

#[derive(Display)]
#[display("NamedFields: x = {}, y = {}, s = {}", x, y, s)]
pub struct NamedFields<'a> {
    x: i32,
    y: i32,
    s: &'a str,
}

#[derive(Display)]
#[display("UnitStruct")]
pub struct UnitStruct;

#[derive(Display)]
#[display("TupleFields: {} {} {}", _0, _1, _2)]
pub struct TupleFields<'a>(i32, i32, &'a str);

fn main() {
    println!("Compiles!");
}
