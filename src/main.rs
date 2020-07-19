use std::fs::File;
use std::io::prelude::*;

use std::io;

extern crate rand;
use rand::Rng;

const ALLOWED_ATTEMPTS: u8 = 5;

enum GameProgress {
  InProgress,
  Won,
  Lost
}

struct Letter {
  character: char,
  revealed: bool
}

// Select a wrod to use
fn selected_word() -> String {
  // Open File
  let mut file = File::open("words.txt").expect("words.txt could not be opened :(");

  // Load file contents into variable
  let mut file_contents = String::new();

  file.read_to_string(&mut file_contents).expect("There was an error reading words.txt :(");

  // Get a word
  let available_words: Vec<&str> = file_contents.trim().split(",").collect();
  let index = rand::thread_rng().gen_range(0, available_words.len());

  return String::from(available_words[index])
}


// Calculate current state of guesses
fn create_letters(word: &String) -> Vec<Letter> {
  let mut letters: Vec<Letter> = Vec::new();

  for character in word.chars() {
    letters.push(Letter {
      character,
      revealed: false
    })
  };

  return letters;
}

// Show current guesses / progress
fn display_progress(letters: &Vec<Letter>) {
  let mut display_string = String::from("Progress:");

  for letter in letters {
    display_string.push(' ');

    if letter.revealed {
      display_string.push(letter.character)
    } else {
      display_string.push('_')
    }

    display_string.push(' ');
  }

  println!("{}", display_string);
}

// Prompt for guess, return it
fn read_user_input_character() -> char {
  let mut user_input: String = String::new();

  match io::stdin().read_line(&mut user_input) {
    Ok(_) => {
      match user_input.chars().next() {
        Some(character) => { return character; }
        None => { return '*'; }
      }
    }
    Err(_) => { return '*'; }
  }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
  let mut all_revealed = true;
  for letter in letters {
    if !letter.revealed {
      all_revealed = false;
    }
  }

  if all_revealed {
    return GameProgress::Won;
  }

  if turns_left > 0 {
    return GameProgress::InProgress;
  }

  return GameProgress::Lost;
}

fn main() {
  let mut turns_left = ALLOWED_ATTEMPTS;
  let selected_word: String = selected_word();
  let mut letters: Vec<Letter> = create_letters(&selected_word);

  loop {
    println!("You have {} turns left", turns_left);
    display_progress(&letters);
    
    println!("Please enter a letter to guess:");
    let user_char = read_user_input_character();

    // User has entered an '*' or input fails
    if user_char == '*' {
      println!("The word was {}", selected_word);
      break;
    }
    // Valid input.  If user has guessed correct letter, prevent them from losing a turn using at_least_one_revealed
    let mut at_least_one_revealed = false;
    for letter in letters.iter_mut() {
      if letter.character == user_char {
        letter.revealed = true;
        at_least_one_revealed = true;
      }
    }
    if !at_least_one_revealed {
      turns_left -= 1;
    }
    
    match check_progress(turns_left, &letters) {
      GameProgress::InProgress => continue,
      GameProgress::Won => {
        println!("\nCongratulations, you've won!  The word was {}", selected_word);
        break;
      }
      GameProgress::Lost => {
        println!("\nYou've lost.  The word was {}.  Please play again.", selected_word);
        break;
      }
    }
  }
}