fn main() {
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

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

fn no_dangle() -> String {
    let s = String::from("sss");
    s
}