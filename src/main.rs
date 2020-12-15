use std::io;

fn main() {
    let mut input = String::new(); // Create a new mutable string to store an input from a user
    
    println!("Please enter a sentence to translate.");

    io::stdin()                         // Get user input
        .read_line(&mut input)          // Reads input and stores in mutable 'input'
        .expect("Failed to read line"); // If reading fails, prints
    
    let mut chars = input.chars().peekable();   // Converts String to char using iterator and stops before consuming char by iterating peek()
    let mut translation = String::new();        // Create a new mutable String to store the translated text

    while let Some(c) = chars.next() {          // Basically, chars.next() is iterating through chars and the loop continues as long as there is 'Some(c)' and ends when 'None'
        let suffix = match c {                  // Create a new variable called suffix, setting suffix to be whatever is returned from 'match c'
            'a' | 'e' | 'i' | 'o' | 'u' => {    // If it's a vowel...
                translation.push(c);            // Push the vowel character to the 'translation' string
                String::from("-hay")            // Set suffix = "-hay"
            }
            #[allow(overlapping_patterns)]
            'a'..='z' | 'A'..='Z' => {          // If it's a consonant...
                format!("-{}ay", c)             // Set suffix = "-{c}ay"
            }
            _ => {
                translation.push(c);            // If it's none of the above (e.g. punctuations or spaces), push it to 'translation'
                continue;                       // Then, continue out of the 'match'
            }
        };                                      // The way it's written, it looks at the first letter to set a suffix unless not a consonant/vowel where it'll instead 'continue'

        while let Some(&c) = chars.peek() {     // Iterating through chars but looking to see the next value to determine whether it should continue.
            match c {                           
                'a'..='z' | 'A'..='Z' => {      // If an alphabetical character...
                    chars.next();               // Skip to the next character (e.g. chars = ['f','o','o'], c = 'o')
                    translation.push(c);        // Push 'c' to 'translation'
                }
                _ => break,                     // If the next character isn't an alphabetical (e.g. punctuation/space) character, break out of 'match'
            }
        }

        translation += &suffix;                 // Add 'suffix' set from earlier to the end of translation (i.e. "oo-fay")
    }                                           // Use a reference because we don't want to take ownership of suffix

    println!("{}", translation);                // Print the final translation of the original input String
}
