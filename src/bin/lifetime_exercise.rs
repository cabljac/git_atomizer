// This works - why?
fn first_word<'a>(s: &'a str) -> &'a str {
    &s[..5]
}

// This doesn't compile - why?
// OWNERSHIP - Can return because we transfer ownership
fn return_string() -> String {
    let s = String::from("hello");
    s // Move ownership out
}

// REFERENCE - Can't return reference to local data
// fn return_ref() -> &str {
//     let s = String::from("hello");
//     &s // Error! s dies here
// }

// REFERENCE - Can return reference to static data
fn return_static_ref() -> &'static str {
    "hello" // Lives forever
}

// This works - different lifetimes
fn longest_with_announcement<'a, 'b>(x: &'a str, _y: &'b str, ann: &str) -> &'a str {
    println!("Announcement: {}", ann);
    x // Always returns x, so only needs 'a lifetime
}

fn main() {
    first_word("Hello World");

    // bad_function();

    longest_with_announcement("placeholder", "placeholder", "Hello World!");

    return_static_ref();
    return_string();
}
