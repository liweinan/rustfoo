use std::collections::HashMap;

fn main() {
    mov_struct_self();

    println!("{:?}", create_a_new_struct());

    use_coerce_static();
    hhash();
    trytry();
    let _bar = barbar();
    one_for_all();
    play_with_point();
    foo();
}

fn mov_struct_self() {
    let mss = MovSelfStruct { val: "Hello, world".into() };

    mss.do_the_move();
    // FAIL: because do_the_move() moved self
    // `mss` moved due to this method call
    // mss.mutate();
}

struct MovSelfStruct {
    val: String,
}

impl MovSelfStruct {
    // Change `self` to `&self` can avoid self moving.
    fn do_the_move(self) {
        // do nothing
        // but self is moved here.
    }

    fn mutate(&self) {
        println!("{:?}", self.val);
    }
}

fn create_a_new_struct() -> AssocStruct {
    AssocStruct::new()
}

#[derive(Debug)]
struct AssocStruct {
    foo: i32
}

impl AssocStruct {
    pub fn new() -> AssocStruct {
        AssocStruct { foo: 42 }
    }
}

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn use_coerce_static() {
    let x = 42;
    println!("coerced_static: {}", coerce_static(&x));
}

fn hhash() -> HashMap<i8, i8> {
    HashMap::new()
}

fn trytry() -> Option<i32> {
    Some(42)
}

fn one_for_all() {
    let bar = Bar(1, 2);
    bar.one_for_all();
}

trait OneForAll {
    fn one_for_all(&self) {
        println!("ONE FOR ALL!");
    }

    fn to_bar(&self) -> Bar {
        Bar(2, 2)
    }
}

impl OneForAll for Bar {}

fn barbar() -> impl OneForAll {
    Bar(42, 42)
}

#[derive(Debug)]
struct Bar(i32, i32);

fn play_with_point() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn no_dangle() -> String {
    let s = String::from("sss");
    s
}

fn foo() {
    println!("Hello World!");
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    let _b = _hello.as_bytes();
    println!("{:?}", _b);
    println!("{:?}", _hello);

    let _r = no_dangle();
    println!("{:?}", _r);

    // try slice
    let x = String::from("cafebabe");
    let y = &x[0..5];
    println!("{:?}", y);

    // https://doc.rust-lang.org/stable/rust-by-example/std/str.html
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }
}
