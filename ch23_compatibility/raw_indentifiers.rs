// This will not compile since the crate `foo` has not been imported and does not actually exist.
// This is an example of using raw identifiers in Rust.
extern crate foo;

fn main() {
    foo::r#try();
}