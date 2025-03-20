use crate::game::GameState;

pub fn display_game_ui(game_state: &GameState) {
  draw_hangman(game_state);
  draw_letterbank(game_state);
  draw_current_word_progress(game_state);
  check_and_draw_win_loss_conditions(game_state);
}

fn draw_hangman(game_state: &GameState) {
  println!("-----");
  println!("|   |");
  // Head
  println!("|   {}", if game_state.hangman_stage >= 1 { "O" } else { "" });
  // Arms
  println!("| {}", if game_state.hangman_stage == 2 { "--|" } else if game_state.hangman_stage >= 3 { "--|--" } else { "" });
  // Left Leg
  println!("|  {}", if game_state.hangman_stage == 4 { "/" } else if game_state.hangman_stage == 5 {"/ \\"} else { "" });
  // Right & Left leg
  println!("|");
  println!("_________\n");
}

fn draw_letterbank(game_state: &GameState){
  let letters_used: String = game_state.letters_used.iter().collect();

  if letters_used.len() > 0 {
    println!("\n\nLetter Bank");
    println!("--------------------------------------------------");
    println!("{}", letters_used);
    println!("--------------------------------------------------");
  }
}

fn draw_current_word_progress(game_state: &GameState) {
  for c in &game_state.current_word_state {
    print!("{} ", c);
  }
}

fn check_and_draw_win_loss_conditions(game_state: &GameState) {
  if game_state.target_word == game_state.current_word_state.iter().collect::<String>() {
    // User has won
    println!("CONGRATS! You guessed the word!!!")
  }

  if game_state.hangman_stage >= 5 {
    // User has lost
    println!("Oh no! You ran out of tries.\n\nThe word was: {}.\n\nTry again!", game_state.target_word);
  }
}