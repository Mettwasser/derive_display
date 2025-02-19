# Info
This is an implementation of a derive macro for the `Display` trait.

I am aware that a lot of people made this prior. I wrote this to better learn how to write proc macros.


# Examples
## Enums
#### Usage
```rs
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
```

#### Expanded
```rs
impl<'a> std::fmt::Display for MyEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyEnum::A { x } => write!(f, "A: {}", x),
            MyEnum::B(_0) => write!(f, "B: {}", _0),
            MyEnum::C => write!(f, "C: Unit",),
        }
    }
}
```

## Structs
### Named fields
```rs
use derive_display::Display;

#[derive(Display)]
#[display("NamedFields: x = {}, y = {}, s = {}", x, y, s)]
pub struct NamedFields<'a> {
    x: i32,
    y: i32,
    s: &'a str,
}


// Expanded:
impl<'a> std::fmt::Display for NamedFields<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let NamedFields { x, y, s } = self;
        write!(f, "NamedFields: x = {}, y = {}, s = {}", x, y, s)
    }
}
```

### Tuple Fields
```rs
#[derive(Display)]
#[display("TupleFields: {} {} {}", _0, _1, _2)]
pub struct TupleFields<'a>(i32, i32, &'a str);


// Expanded:
impl<'a> std::fmt::Display for TupleFields<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let TupleFields(_0, _1, _2) = self;
        write!(f, "TupleFields: {} {} {}", _0, _1, _2)
    }
}
```

### Unit Structs
```rs
#[derive(Display)]
#[display("UnitStruct")]
pub struct UnitStruct;


// Expanded:
impl std::fmt::Display for UnitStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitStruct",)
    }
}
```