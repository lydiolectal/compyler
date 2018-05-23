struct StructStruct {
    foo: String,
}

struct TupleStruct(u32);

let x = TupleStruct(1);

struct Empty;

let x = Empty;

enum Foo {
    VariantA,
    VariantB(i32),
    VariantC{y: i32},
}

let x = Foo::VariantA;
let x = Foo::VariantB(1);
let x = Foo::VariantC{y: 1};
let x = StructStruct{foo: String::new()};
