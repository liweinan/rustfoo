struct Foo {
    x: i32,
    s: String,
    opt: Option<i32>,
}

struct Bar(i32, i32);

fn play_with_struct() -> Foo {
    let d = Foo {
        x: 4,
        s: String::from("hello world"),
        opt: None,
    };

    return d;
}

fn main() {
    let foo = play_with_struct();
    println!("{:?}", foo.x);
    println!("{:?}", foo.s);
    println!("{:?}", foo.opt);

    let bar = Bar(42,24);
    println!("{:?}", bar.0);
    println!("{:?}", bar.1);
}