use crate::randomwordgenerator::get_random_word;
use crate::display::display_game_ui;
use std::io::{self, Read};
pub struct GameState {
  pub hangman_stage: i32, 
  pub target_word: String,
  pub current_word_state: Vec<char>,
  pub letters_used: Vec<char>,
}

impl GameState {
  pub fn update_current_word_state(&mut self, index: usize, letter: char) {
    self.current_word_state[index] = letter; 
  }

  pub fn update_hangman_stage(&mut self, stage: i32) {
    self.hangman_stage = stage;
  }

  pub fn update_letters_used(&mut self, letter: char) {
    self.letters_used.push(letter);
  }
}

pub fn start_game() {
  let random_word = match get_random_word() {
    Ok(word) => word,
    Err(e) => { 
      eprintln!("Could not fetch random word, aborting game. Error: {e}");
      return ();
    }
  };

  let mut game_state = init_game_state(random_word);

  loop {
    display_game_ui(&game_state);

    let current_word: String = game_state.current_word_state.iter().collect();

    // Win condition
    if game_state.target_word == current_word {
      break;
    }

    // Loss condition
    if game_state.hangman_stage >= 5 {
      break;
    }

    let input_letter = loop { 
      match get_user_input() {
        Ok(c) => {
          if game_state.letters_used.contains(&c) {
            println!("You have already used {c}, choose another letter");
          } else {
            break c;
          }
        },
        Err(e) => { println!("{e}") }
      }
    };

    let idxs = get_matching_current_word_idxs(&input_letter, &game_state.target_word);
    if idxs.len() > 0 {
      // Update current word vector
      for idx in idxs {
        game_state.update_current_word_state(idx, input_letter);
      }
    } else {
      // Increase hangman stage
      game_state.update_hangman_stage(game_state.hangman_stage + 1);
    }

    game_state.update_letters_used(input_letter);
  }
}

fn get_user_input() -> Result<char, String> {
  println!("\n\nEnter a letter: ");
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .map_err(|e| e.to_string())?;

  if input.trim().len() > 1 {
    return Err("Please enter a single character.".to_string());
  }

  let c = input.trim().chars().next().ok_or("No character entered")?;

  if !c.is_alphabetic() {
    return Err("Please enter an alphabetic character from a-z".to_string());
  }

  Ok(c)
}

fn get_matching_current_word_idxs(c: &char, target_word: &str) -> Vec<usize> {
  target_word.chars()
    .enumerate()
    .filter(|(_, ch)| ch == c)
    .map(|(i, _)| i)
    .collect()
}

pub fn init_game_state(target_word: String) -> GameState {
  let word_len = target_word.len();
  GameState { hangman_stage: 0, target_word: target_word, current_word_state: vec!['_'; word_len], letters_used: vec![] }
}

