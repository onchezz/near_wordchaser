mod utils;

// extern crate quad_rand;

mod json_data;
use json_data::json;
// use contract::json;

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, log, near_bindgen,
  serde::{Deserialize, Serialize},
  serde_json, *,
};

use quad_rand as rand;

#[near_bindgen]
#[derive(
  PanicOnDefault, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone,
)]
#[serde(rename_all = "camelCase")]
pub struct Word {
  pub word: String,
  pub meaning: String,
  pub example: String,
}
#[near_bindgen]
impl Word {
  pub fn get_word_json(data: String) -> Self {
    //saving words to a vector of type ::word which is a struct
    let words: Vec<Word> = serde_json::from_str(data.as_str()).expect("error giving json");

    let random_index = rand::gen_range(0, words.len());
    //returning the generated word
    words[random_index].clone()
  }

  /*generating a string with the random word hidden on the example usange of the word */
  pub fn creat_example_string(&mut self) -> String {
    let word_example = &self.example;
    let mut letters = Letters::get_letters(&mut self.word);
    let unknown_word = Letters::display_progress(&mut letters);

    let generated_word = word_example.replace(&self.word, unknown_word.as_str());

    generated_word
  }
}

#[near_bindgen]
#[derive(PanicOnDefault, Serialize, Deserialize, Debug, Clone)]
pub struct Letters {
  pub letter: char,
  pub is_revealed: bool,
}
#[near_bindgen]
impl Letters {
  //getting letters from the generated word
  pub fn get_letters(word: &mut String) -> Vec<Letters> {
    let mut chars: Vec<Letters> = Vec::new();
    //destracturing and looping the word letters and  saving them in a vector of type letters
    for c in word.chars() {
      chars.push(Self {
        letter: c,
        is_revealed: false,
      });
    }
    //returning the generated letters
    chars
  }

  /*this functions takes in a vector of letters and creats a display string of letter depending on if it is revealed or not */
  fn display_progress(letters: &mut Vec<Letters>) -> String {
    let mut display_string = String::from("  ");
    for l in letters {
      if l.is_revealed {
        display_string.push(l.letter);
      } else {
        display_string.push('_');
      }
    }
    display_string
  }
}

pub mod functionality {

  use super::*;
  /*getting data from json file and returing strings  */
  pub fn get_file_data() -> String {
    let data = json::data();
    data
  }

  /*Reading user input and returns a character  */
  pub fn read_user_input() -> char {
    let user_input = env::input().unwrap();
    let input = String::from_utf8(user_input).unwrap();
    let user_char = input.chars().next().expect("failed generating user letter");
    user_char
  }

  pub fn play() {
    let log_message = format!("    Chase  the word  ");

    env::log_str(log_message.as_str());
    let data = get_file_data();
    let mut word = Word::get_word_json(data);

    let mut letters = Letters::get_letters(&mut word.word);
    let example = Word::creat_example_string(&mut word);
    let log_message = format!(
      "Meaning :  \n{}        \nExample:  \n{}",
      word.meaning, example
    );

    env::log_str(log_message.as_str());
    let mut turns_available = word.word.len();
    let mut tries = 0;
    let mut reveal_count = 0;
    let letter_reveared = format!("number of letters revealed {}", reveal_count);
    let mut input: char;
    loop {
      let log_meassage = format!("Turns remaining  {} ", turns_available);

      env::log_str(log_meassage.as_str());
      let display = Letters::display_progress(&mut letters);

      log!("Progress {}", display);
      // env::log_str(message.as_bytes());
      log!("Enter a letter ");

      input = read_user_input();

      let mut atleast_found_a_char: bool = false;
      for l in letters.iter_mut() {
        if l.letter == input {
          l.is_revealed = true;
          atleast_found_a_char = true;
          reveal_count += 1;
        }
      }

      if !atleast_found_a_char {
        if turns_available > 0 {
          turns_available -= 1;
          tries += 1;
        }
        if turns_available == 0 {
          log!("Sorry you lost {}:  turns  left", turns_available);
          env::log_str(letter_reveared.as_str());

          break;
        }
      }

      /*checking if all letters of the word are reaveled and returning is true*/
      let is_valid = letters
        .iter()
        .zip(letters.iter())
        .all(|(_, c)| c.is_revealed);

      if is_valid {
        log!("Congrats you won the game ");
        let tries_msg = format!("you failed  {}:times  out of {}", tries, word.word.len());
        env::log_str(tries_msg.as_str());

        break;
      }
      if input == '!' {
        env::log_str(letter_reveared.as_str());
        log!("exiting Game ");
        break;
      }
      if input == '>' {
        next();

        break;
      }
    }

    log!("The word was {}", word.word);
  }
  fn next() {
    log!("Skiping word  >>>> ");
    play();
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use near_sdk::{test_utils::*, testing_env};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "contract".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  fn test() {}
}
