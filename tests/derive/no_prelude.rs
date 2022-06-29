#![no_implicit_prelude]

#[derive(::hexz::Bundle)]
struct Foo {
    foo: (),
}

#[derive(::hexz::Bundle)]
struct Bar<T> {
    foo: T,
}

#[derive(::hexz::Bundle)]
struct Baz;

#[derive(::hexz::Query)]
struct Quux<'a> {
    foo: &'a (),
}

fn main() {}
