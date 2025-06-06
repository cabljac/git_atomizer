// A struct that holds a reference to some text and can scan through it
struct Scanner<'a> {
    text: &'a str,
    position: usize,
}

fn first_word_from_longer<'a, 'b>(s1: &mut Scanner<'a>, s2: &mut Scanner<'b>) -> Option<String> {
    if s1.text.len() > s2.text.len() {
        s1.next_word().map(|s| s.to_string())
    } else {
        s2.next_word().map(|s| s.to_string())
    }
}

impl<'a> Scanner<'a> {
    fn new(text: &'a str) -> Self {
        Scanner { text, position: 0 }
    }

    // TODO: Implement this method
    // It should return the next word and advance the position
    fn next_word(&mut self) -> Option<&'a str> {
        let remaining = &self.text[self.position..];

        let trimmed = remaining.trim_start();

        if trimmed.is_empty() {
            return None;
        }

        let spaces_skipped = remaining.len() - trimmed.len();

        self.position += spaces_skipped;

        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());

        let word = &self.text[self.position..self.position + end];

        self.position += end;

        Some(word)

        // match trimmed.find(char::is_whitespace) {
        //     Some(end) => {
        //         let word = Some(&self.text[self.position..(self.position + end)]);
        //         self.position += end;
        //         return word;
        //     }
        //     None => {
        //         self.position += trimmed.len();
        //         Some(&trimmed)
        //     }
        // }
    }
}

struct ScannerThatOwnsItsText {
    text: String,
    position: usize,
}

// When Rust infers lifetimes automatically:
// Rust's "lifetime elision rules" handle common patterns:

// One input reference → output gets same lifetime
// rustfn foo(&self) -> &str
// Rust sees: fn foo<'a>(&'a self) -> &'a str

// Multiple inputs, one is &self → output tied to self
// rustfn foo(&self, other: &str) -> &str
// Rust sees: fn foo<'a, 'b>(&'a self, other: &'b str) -> &'a str

impl<'a> ScannerThatOwnsItsText {
    fn new(text: String) -> Self {
        ScannerThatOwnsItsText { text, position: 0 }
    }

    fn next_word(&'a mut self) -> Option<&'a str> {
        let remaining = &self.text[self.position..];

        let trimmed = remaining.trim_start();

        if trimmed.is_empty() {
            return None;
        }

        let spaces_skipped = remaining.len() - trimmed.len();

        self.position += spaces_skipped;

        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());

        let word = &self.text[self.position..self.position + end];

        self.position += end;

        Some(word)
    }
}

fn main() {
    // Challenge 1: Make this compile and work
    let text = String::from("hello rust world");
    let mut scanner = Scanner::new(&text);

    while let Some(word) = scanner.next_word() {
        println!("Word: {}", word);
    }

    // Challenge 2: Why doesn't this compile? Fix it!

    // Answer: It doesn't compile because we're trying to get the reference word from this scope
    //  (by the way i didnt realize scopes return like this, or rather i'm only just realizing you can return from a scope without a function!
    // is that specific to Rust? )
    // anyway what we should do is below
    // let word = {
    //     let text2 = String::from("temporary text");
    //     let mut scanner2 = Scanner::new(&text2);
    //     scanner2.next_word()
    // };

    // Option 1 is we could move word and the print statement into the block.
    {
        let text2 = String::from("temporary text");
        let mut scanner2 = Scanner::new(&text2);
        let word = scanner2.next_word();
        println!("Word from block: {:?}", word);
    };
    //  Option 2 is we could move the scanner out of the block
    let text2 = String::from("temporary text");
    let mut scanner2 = Scanner::new(&text2);
    let word = { scanner2.next_word() };
    println!("Word from block: {:?}", word);

    // Option 3: Return owned data instead of a reference
    //  This keeps the scanner in the scope we wanted, and extracts the word from that scope as an Option<String>
    let word = {
        // String stores data on the heap, creates a reference
        let text2 = String::from("temporary text");
        let mut scanner2 = Scanner::new(&text2);
        scanner2.next_word().map(|s| s.to_string()) // Convert &str to String
    };
    println!("Word from block: {:?}", word); // This is Option<String> now

    // Challenge 3: Create a function that takes two scanners
    // with potentially different lifetimes and returns the
    // first word from the longer text

    let text1 = String::from("hello world"); // 11 chars
    let text2 = String::from("rust programming"); // 16 chars
    let mut scanner1 = Scanner::new(&text1);
    let mut scanner2 = Scanner::new(&text2);

    let result = first_word_from_longer(&mut scanner1, &mut scanner2).unwrap();

    println!("{} is the longest", result);

    // Bonus: Can you make a scanner that owns its text instead?
    let text = String::from("hello world i will be owned by the scanner");
    let mut owned_scanner = ScannerThatOwnsItsText::new(text);

    let word_from_owned_scanner = owned_scanner.next_word().unwrap_or_else(|| "");

    println!("word from the owned scanner: {}", word_from_owned_scanner)
}
