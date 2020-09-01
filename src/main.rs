fn main() {
    play_with_point();
    foo();
}

fn play_with_point() {
    let p1 = Point {x:5, y:10.4};
    let p2 = Point {x:"Hello", y: 'c'};
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
