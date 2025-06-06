fn lifetime_example() {
    let string1 = String::from("hello");
    let result;
    let string2 = String::from("world");
    {
        // This errors in the s2 argument because string2 began life within this scope,
        // and will die before the println! line is executed (line 10) outside the scope.
        // the fix is to move string2 instantiation to before this scope/block
        result = longest(&string1, &string2);
        println!("Longest: {}", result);
    }
    // uncomment this to see the error
    // println!("Result after scope: {}", result);
}

// Annotation "This returns a reference which will die when one of s1 or s2 dies"
// OR is it more accurate to say that the lifetime of s1 and s2 is just an upper bound of the lifetime of longest(&s1,&s2)?
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    lifetime_example();
}
