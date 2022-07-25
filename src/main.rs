use std::io;

/*
* Hangman
*
* Ask for a word and it will do hangman with it.
* Hangman Model, the word area, and the used letters area
*/

enum Model {
    Tree,
    Head,
    Body,
    Arms,
    Legs,
}

impl Model {
    fn value(&self) -> &str {
        let tree = "
         |--------
         |       |
         |
         |
         |
         |
         |
    ___________";

        let head = "
         |--------
         |       |
         |       O
         |
         |
         |
         |
    ___________";

        let body = "
         |--------
         |       |
         |       O
         |       |
         |       |
         |
         |
    ___________";

        let arms = "
         |--------
         |       |
         |       O
         |      /|\\
         |       |
         |
         |
    ___________";

        let legs = "
         |--------
         |       |
         |       O
         |      /|\\
         |       |
         |      / \\
         |
         |
    ___________";

        match self {
            Model::Tree => tree,
            Model::Head => head,
            Model::Body => body,
            Model::Arms => arms,
            Model::Legs => legs,
        }
    }
}

// Accepts a letter, the word, and blank word. Modifies the blank word and returns true if the
// letter is included in the word, else false
fn check_letter(word: &str, blank_word: &mut String, letter: &str) -> bool {
    if word.contains(letter) {
        for (j, i) in word.chars().enumerate() {
            if i.to_string() == letter {
                blank_word.replace_range(j..j + 1, letter);
            }
        }
        true
    } else {
        false
    }
}

fn main() {
    // Ask for word
    let mut word = String::new();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Enter a word: ");
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    // Create a blank word that matches the word
    let mut blank_word = String::new();
    for _i in 0..word.chars().count() - 1 {
        blank_word.push('_');
    }

    // Stores the letters we used
    let mut used_letters: Vec<String> = Vec::new();

    // Number of lives left
    let mut lives = 5;

    // Display the Model and ask for a letter
    let mut model = Model::Tree;
    loop {
        // Clears the screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // Print the model
        println!("{}", model.value());

        // Print the blank word
        println!();
        println!("                      {}", blank_word);

        // Print used letters
        println!();
        for c in &used_letters {
            print!("{}  ", c);
        }
        println!();

        // Check if we won
        if word.trim() == blank_word {
            println!();
            println!("You Win");
            break;
        }

        // Ask for a letter
        println!();
        println!("Enter a letter");
        let mut letter = String::new();
        io::stdin()
            .read_line(&mut letter)
            .expect("Failed to read character");
        letter = letter.trim().to_string();
        // and add that letter to used_letters
        used_letters.push(letter.to_string());

        // Check if that letter is in the word and modify blank word
        let letter_in: bool = check_letter(&word, &mut blank_word, &letter);

        // Change the number of lives based on letter_in
        if !letter_in {
            lives -= 1;
        }

        // Change the model based on the number of lives
        match lives {
            5 => model = Model::Tree,
            4 => model = Model::Head,
            3 => model = Model::Body,
            2 => model = Model::Arms,
            1 => model = Model::Legs,
            0 => println!("You are dead"),
            _ => println!("Now you are super dead"),
        }

        // Check if we ran out of lives
        if lives == 0 {
            println!("Game Over. The word was {}", word);
            break;
        }
    }
}
