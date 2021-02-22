#![feature(assoc_char_funcs)]

use std::collections::HashMap;

fn main() {
    try_closures();

    // let divisor = -(268_i32.div_euclid(60));
    // println!("divisor is {}" , divisor);
    //
    // let divisor2 = (-268_i32).div_euclid(60);
    // println!("divisor2 is {}" , divisor2);
    //
    // let divisor3: i32 = -268;
    // println!("divisor3 is {}" , divisor3.div_euclid(60));
    //
    // println!("divisor4 is {}" , (-268 as i32).div_euclid(60));
    //
    // use_monotone_increasing_digits();
    // use_monotone_increasing_digits();
    // regexp();
    // sort_median();
    // intointo();
    // range_two_dots();
    // use_gfunc();
    // generic();
    // mov_struct_self();
    //
    // println!("{:?}", create_a_new_struct());
    //
    // use_coerce_static();
    // hhash();
    // trytry();
    // let _bar = barbar();
    // one_for_all();
    // play_with_point();
    // foo();
}

fn try_closures() {
    let immut_val = String::from("immut");

    let fn_closure = || {
        println!("{}", immut_val.len());
    };

    call_fn(fn_closure);
    call_fn(fn_closure);
    call_fn(fn_closure);

    let to_move_val = String::from("moved_val");
    let fn_once = || {
        let moved_val = to_move_val;
        println!("{}", moved_val.len());
    };

    call_fn_once(fn_once);
    // call_fn_once(fn_once); note: closure cannot be moved more than once as it is not `Copy` due to moving the variable `to_move_val` out of its environment
}

fn call_fn_once<F>(f: F)
    where F: FnOnce(),
{
    f();
}

fn call_fn<F>(f: F)
    where F: Fn(), {
    f();
}

fn use_monotone_increasing_digits() {
    println!("{:?}", monotone_increasing_digits(10));
    println!("{:?}", monotone_increasing_digits(21));
    println!("{:?}", monotone_increasing_digits(332));
    println!("{:?}", monotone_increasing_digits(1234));
    println!("{:?}", monotone_increasing_digits(528357107));
    println!("{:?}", monotone_increasing_digits(526115566));
    println!("{:?}", monotone_increasing_digits(882930776));
}

pub fn monotone_increasing_digits(n: i32) -> i32 {
    let str_n = n.to_string();
    let mut i = 1;
    let mut chars: Vec<_> = str_n.chars().collect();
    while i < str_n.len() && chars[i - 1] <= chars[i] {
        i += 1;
    }

    if i < str_n.len() {
        while i > 0 && chars[i - 1] > chars[i] {
            // chars[i - 1] = char::from_u32(chars[i - 1] as u32 - 1).unwrap();
            let m = chars[i - 1] as u8 - 1;
            chars[i - 1] = m as char;
            i -= 1;
        }
        for i in i + 1..str_n.len() {
            chars[i] = '9';
        }
    }

    let str: String = chars.iter().collect();
    str.parse::<i32>().unwrap()
}

pub fn monotone_increasing_digits2(n: i32) -> i32 {
    let mut r = 0;
    for i in (0..=n).rev() {
        // let (_r, ok) = monotone(i, i);
        let (_r, ok) = monotone2(i);
        if ok {
            r = _r;
            break;
        }
    }

    r
}

pub fn monotone2(start_n: i32) -> (i32, bool) {
    if start_n < 10 { return (start_n, true); }
    if start_n == 10 { return (9, true); }

    let mut current_n = start_n;

    loop {
        if current_n > 0 {
            let current_digit = current_n % 10;
            let next_digit = current_n / 10 % 10;

            if current_digit >= next_digit {
                if current_n / 100 == 0 {
                    return (start_n, true);
                } else {
                    current_n = current_n / 10
                };
            } else {
                return (-1, false);
            }
        } else {
            return (start_n, true);
        };
    }
}

pub fn monotone(start_n: i32, current_n: i32) -> (i32, bool) {
    if start_n < 10 { return (start_n, true); }
    if start_n == 10 { return (9, true); }

    return if current_n > 0 {
        let current_digit = current_n % 10;
        let next_digit = current_n / 10 % 10;

        if current_digit >= next_digit {
            return if current_n / 100 == 0 {
                (start_n, true)
            } else {
                monotone(start_n, current_n / 10)
            };
        } else {
            (-1, false)
        }
    } else {
        (start_n, true)
    };
}

fn regexp() {
    println!("{:?}", is_match("ab".into(), ".*".into()));
    println!("{:?}", is_match("aa".into(), "a".into()));
    println!("{:?}", is_match("aa".into(), "a*".into()));
}

pub fn is_match(s: String, p: String) -> bool {
    _is_match(&s, &p)
}

fn _is_match(s: &String, p: &String) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }

    let first_match = !s.is_empty() && (s.chars().nth(0) == p.chars().nth(0) || p.chars().nth(0).unwrap() == '.');

    return if p.len() >= 2 && p.chars().nth(1).unwrap() == '*' {
        _is_match(s, &String::from(&p[2..])) || (first_match && _is_match(&String::from(&s[1..]), p))
    } else {
        first_match && _is_match(&String::from(&s[1..]), &String::from(&p[1..]))
    };
}

fn sort_median() {
    let vec1 = vec![1, 5, 10, 2, 15];
    let vec2 = vec![1, 5, 10, 2, 15];
    find_median_sorted_arrays(vec1, vec2);
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut cons = [&nums1[..], &nums2[..]].concat();
    cons.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", cons);

    let size = cons.len();

    return if size == 1 {
        cons[0] as f64
    } else if size == 2 {
        (cons[0] + cons[1]) as f64 / 2.0
    } else if size == 3 {
        cons[1] as f64
    } else if size % 2 == 1 {
        cons[size / 2] as f64
    } else if size % 2 == 0 {
        (cons[size / 2] + cons[size / 2 - 1]) as f64 / 2.0
    } else {
        42.0
    };
}

fn intointo() {
    let s: String = "Hello, world!".into();
    println!("{:?}", s);
}


fn range_two_dots() {
    println!("{:?}", 1..10);
    println!("{:?}", (..6));
    println!("{:?}", 4..);
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    println!("{:?}", u);
}

fn use_gfunc() {
    println!("{:?}", gfunc(2, 3));
}

fn gfunc<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

fn generic() {
    let a = GenericPoint { x: 100, y: 20 };
    let b = GenericPoint { x: 1.0, y: 2.2 };
    // mismatched types
    // let c = GenericPoint { x: 2.0, y: 2 };
    println!("{:?}", a);
    println!("{:?}", b);
}

#[derive(Debug)]
// abstract / placeholder type
struct GenericPoint<T> {
    x: T,
    y: T,
}

fn mov_struct_self() {
    let mss = MovSelfStruct { val: "Hello, world".into() };

    // UPDATE: move out
    let mss2 = mss.do_the_move();
    // FAIL: because do_the_move() moved self
    // `mss` moved due to this method call
    // use the moved var
    mss2.mutate();
}

struct MovSelfStruct {
    #[allow(dead_code)]
    val: String,
}

impl MovSelfStruct {
    // Change `self` to `&self` can avoid self moving.
    // UPDATE: move back
    fn do_the_move(self) -> MovSelfStruct {
        // do nothing
        // but self is moved here.
        self
    }

    #[allow(dead_code)]
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
