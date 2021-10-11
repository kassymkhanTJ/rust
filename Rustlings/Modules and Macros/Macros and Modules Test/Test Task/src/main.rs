#[macro_use]
mod foo {
    macro_rules! my_macro {
        ($s: tt) => (concat!("Hello ", $s))
    }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
