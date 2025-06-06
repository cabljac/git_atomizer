// A struct that holds a reference to some text and can scan through it
struct Scanner<'a> {
    text: &'a str,
    position: usize,
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

fn main() {
    // Challenge 1: Make this compile and work
    let text = String::from("hello rust world");
    let mut scanner = Scanner::new(&text);

    while let Some(word) = scanner.next_word() {
        println!("Word: {}", word);
    }

    // Challenge 2: Why doesn't this compile? Fix it!
    // let word = {
    //     let text2 = String::from("temporary text");
    //     let mut scanner2 = Scanner::new(&text2);
    //     scanner2.next_word()
    // };
    // println!("Word from block: {:?}", word);

    // Challenge 3: Create a function that takes two scanners
    // with potentially different lifetimes and returns the
    // first word from the longer text

    // Bonus: Can you make a scanner that owns its text instead?
}
