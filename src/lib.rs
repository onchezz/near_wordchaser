use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, log, near_bindgen,
  serde::{Deserialize, Serialize},
  serde_json,
};

mod data;
mod how_play;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug)] //PartialEq
pub struct Info {
  info: String,
  description: String,
  method: String,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "camelCase")]
pub struct Vocabulary {
  pub example: String,
  pub meaning: String,
  pub word: String,
}
#[near_bindgen]
impl Vocabulary {
  pub fn create_unknown_word(&mut self) -> Vocabulary {
    let mut letters = Letter::get_letters(&mut self.word);
    let display_string = Letter::display_progress(&mut letters);
    let replace_example = self.example.replace(&self.word, &display_string);
    let unknown_word = Vocabulary {
      example: replace_example,
      meaning: self.meaning.clone(),
      word: display_string,
    };
    unknown_word
  }
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
  Completed,
  Inprogress,
  Failed,
}

#[derive(Debug, Serialize, Deserialize)]
#[near_bindgen]
pub struct Letter {
  pub letter: char,
  pub is_revealed: bool,
}
#[near_bindgen]
impl Letter {
  /*getting letters from the generated word*/
  pub fn get_letters(word: &mut String) -> Vec<Letter> {
    let mut chars: Vec<Letter> = Vec::new();
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
  pub fn display_progress(letters: &mut Vec<Letter>) -> String {
    let mut display_string = String::from("");
    for l in letters {
      display_string.push(' ');
      if l.is_revealed {
        display_string.push(l.letter);
      } else {
        display_string.push('_');
      }
    }
    display_string
  }
  /*this function matches the userinput and the random word  and finds which  letter is revealed ans sets is revealed to true */
  pub fn reveal_letters(user_random: &mut String, user_input: String) -> Vec<Letter> {
    let mut random_word_letters = Letter::get_letters(user_random);
    /*user input word  */
    let guessed_word_letters: Vec<char> = user_input.chars().collect();

    for l in random_word_letters.iter_mut() {
      let letter = l.letter;
      for uin in guessed_word_letters.iter() {
        if &letter == uin {
          l.is_revealed = true;
        }
      }
    }
    random_word_letters
  }
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub struct CompletedWord {
  word: String,
  status: String,
  trials_completed_at: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
  userid: String,
  guess: String,
  word_progress: String,
  completed: Vec<CompletedWord>,
  turns: u64,
  is_revealed: bool,
}
impl Default for Player {
  fn default() -> Self {
    Self {
      userid: String::from("word.test"),
      guess: String::from("welcome 💖"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      // UnorderedMap::new(b"w"),
      turns: 0,
      is_revealed: false,
    }
  }
}

#[near_bindgen]
impl Player {
  /*this a function to get data from a json file imitating a REST API for random words */
  fn get_data(&mut self) -> Option<Vec<Vocabulary>> {
    /*getting data from json  data  */
    let data = data::json_data();
    //saving words to a vector of type ::word wh&&ich is a struct
    let words: Vec<Vocabulary> = serde_json::from_value(data).expect("erro giving json");
    Some(words)
  }

  fn playinfo(&mut self) -> Option<Vec<Info>> {
    let playinfo = how_play::how_to_play_json();
    let how_play: Vec<Info> = serde_json::from_value(playinfo).expect("unable to get playing info");
    Some(how_play)
  }

  pub fn how_to_play(&mut self) -> Result<Vec<Info>, &str> {
    let mut how_to_play: Vec<Info> = Vec::new();
    let words = self.get_data().unwrap();
    let info = self.playinfo();
    let account = env::signer_account_id();
    let user = String::from(account);
    /*
     *this function creates a  help based if the user exists
     *if the user exists  returns how to play without a  welcome message
     */
    match self.userid == user {
      true => Ok(info.unwrap()),
      false => match info {
        Some(info) => {
          let desc = format!(
            "To start the game call random_word and choose any number from 0 and {}",
            words.len() - 1
          );
          let wel = Info {
            info: format!("welcome  {}", user),
            description: desc,
            method: "random_word".to_string(),
          };
          how_to_play.push(wel);
          for i in 0..info.len() {
            how_to_play.push(info[i].clone());
          }
          Ok(how_to_play)
        }
        None => Err("Sorry failed to get help"),
      },
    }
  }

  pub fn view_uncompleted_words(&mut self) -> Option<Vec<Vocabulary>> {
    /*this function creates vocabularies with hidden words for the player crack
    eg ..
     {
        example: 'The committee worked in    _ _ _ _ _ _ on the bill, and it eventually passed.',
        meaning: 'concurrence of opinion',
        word: '   _ _ _ _ _ _'
      },
      */
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    let words = self.get_data();
    match words {
      Some(mut words) => {
        let mut display_unknown_words: Vec<Vocabulary> = Vec::new();

        for word in words.iter_mut() {
          let unknown_word = word.create_unknown_word();
          if !self.completed.is_empty() {
            if self.userid == user {
              for compl in self.completed.iter_mut() {
                if word.word == compl.word {
                  let insert_word = word.to_owned();
                  display_unknown_words.push(insert_word);
                }
              }
            }
          } else {
            display_unknown_words.push(unknown_word);
          }
        }
        log!("please chooose a number between 0 and {}", words.len() - 1);
        Some(display_unknown_words)
      }

      None => {
        log!("failed to create words");
        None
      }
    }
  }

  /*this is a function to generate a random word with range to index to play with  */
  pub fn random_word(&mut self, num: usize) -> Result<Vocabulary, Info> {
    //saving words to a vector of type ::word which is a struct
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    if user != self.userid {
      self.userid = user;
    }
    match self.get_data() {
      Some(words) => match words.get(num) {
        Some(w) => {
          let mut w = w.clone();
          let unknown_word = w.create_unknown_word();
          self.guess = w.word;
          self.turns = self.guess.len() as u64;
          self.is_revealed = false;
          return Ok(unknown_word);
        }
        None => {
          let msg = format!("please chooose a number between 0 and {}", words.len() - 1);
          let i = Info {
            info: "Error".to_string(),
            description: msg,
            method: "random_word".to_string(),
          };
          return Err(i);
        }
      },
      None => {
        let msg = format!("failed to get words from json ",);
        let i = Info {
          info: "Error".to_string(),
          description: msg,
          method: "random_word".to_string(),
        };
        return Err(i);
      }
    }
  }
  /* gets the usser solution to the random word created*/
  pub fn check_solution(&mut self, word: String) -> Result<String, Info> {
    let account_id = env::signer_account_id();
    let current_user = String::from(account_id);
    /*checks if the user id is equal to the saved id  from the randon word function  */
    match current_user == self.userid {
      true => {
        let mut w = self.guess.clone();
        let mut revealed_letters = Letter::reveal_letters(&mut w, word);
        self.word_progress = Letter::display_progress(&mut revealed_letters);
        /*it takes all the iterator in the letters vector and creats a single iterator thats iterates through all of the letters cheking if the user revealed the letter  and returns true if the letters are revealed*/

        match self.check_progress(self.turns, &revealed_letters) {
          Status::Completed => {
            self.completed.push(CompletedWord {
              word: String::from(&self.guess),
              status: String::from("completed"),
              trials_completed_at: String::from(format!("{} trials", self.turns)),
            });

            let msg = format!("Huree you won !!!",);
            Ok(msg)
          }
          Status::Inprogress => {
            let msg = format!(
              "Progress {} tials remaining {}:trials",
              &self.word_progress, &self.turns,
            );
            self.turns -= 1;
            Ok(msg)
          }
          Status::Failed => {
            let msg = format!(
              "Sorry you Lost the Game  {} tials remaining {}:trials",
              &self.word_progress, &self.turns
            );
            Ok(msg)
          }
        }
      }
      false => {
        let err = format!("Sorry try again to geuss a random Word again",);
        Err(Info {
          info: "sorry".to_string(),
          description: err,
          method: "random_word".to_string(),
        })
      }
    }
  }

  /*checks the progress of the  game and returns  the status  */
  fn check_progress(&mut self, turns: u64, letters: &Vec<Letter>) -> Status {
    self.is_revealed = letters
      .iter()
      .zip(letters.iter())
      .all(|(_, c)| c.is_revealed);

    if self.is_revealed == true {
      return Status::Completed;
    } else if turns < 1 {
      return Status::Failed;
    } else {
      return Status::Inprogress;
    }
  }
  /*gets the complete words  */
  pub fn view_completed(&self) -> Result<&Vec<CompletedWord>, String> {
    let account_id = env::signer_account_id();
    let user = String::from(account_id);
    match self.userid == user {
      true => {
        if !&self.completed.is_empty() {
          Ok(&self.completed)
        } else {
          Err(String::from("Sorry You have not yet completed any words"))
        }
      }
      false => Err(String::from(
        "No completed  words available for this account",
      )),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::test_utils::VMContextBuilder;
  // use near_sdk::MockedBlockchain;
  // use near_sdk::{log, testing_env, VMContext};
  use near_sdk::{log, testing_env, AccountId, VMContext};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "contract".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContext {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder.build()
  }

  #[test]
  fn test_create_unknown_word() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let mut word = Vocabulary {
      example: "deploying a smartcontract to a near blockchain is easy and the smartcontract is faster other chains.".to_string(),
      meaning: "code that lives on the blockchain".to_string(),
      word: "smartcontract".to_string(),
    };

    let hidden_word = word.create_unknown_word();
    log!("{:#?}", hidden_word);

    assert_eq!(hidden_word.example, "deploying a  _ _ _ _ _ _ _ _ _ _ _ _ _ to a near blockchain is easy and the  _ _ _ _ _ _ _ _ _ _ _ _ _ is faster other chains.");
    assert_eq!(hidden_word.meaning, "code that lives on the blockchain");
    assert_eq!(hidden_word.word, " _ _ _ _ _ _ _ _ _ _ _ _ _");
  }

  #[test]
  fn test_get_letters() {
    /*
    This function Generates a vector of  characters  and creates a Struct of type letter for each  char in the letter
      [
       { letter: 'N', is_revealed: false },
       { letter: 'e', is_revealed: false },
       { letter: 'a', is_revealed: false },
       { letter: 'r', is_revealed: false }
      ]
    */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let mut word = String::from("Near");
    let letters = Letter::get_letters(&mut word);

    assert_eq!(letters.len(), 4);
  }

  #[test]
  fn test_display_progress() {
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    /*this function changes the state of  the letter  where its known  an replaces the rest with _ _ _*/
    let mut word = String::from("Near");
    let mut letters = Letter::get_letters(&mut word);

    //changing the state at index 0 and 3 to show letters
    for i in 0..letters.len() {
      if i == 0 || i == 3 {
        letters[i].is_revealed = true;
      }
    }
    let chars = &mut letters;

    let display_string = Letter::display_progress(chars);

    assert_eq!(display_string, " N _ _ r")
  }

  #[test]
  fn test_if_reveales_correct_letters() {
    /*this function tests if the  words that the user enters is equal to  word that the user entered and if its not equal it reveals only the letters that are the same in both the input ana the randomly generated word   */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let mut user_random_chosen_word = "contract".to_string();
    let user_input = "contact".to_string();
    let mut revealed = Letter::reveal_letters(&mut user_random_chosen_word, user_input);
    let known_letters = Letter::display_progress(&mut revealed);

    assert_eq!(known_letters, " c o n t _ a c t")
  }

  #[test]
  fn get_data_from_jsons() {
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    /* this function tests if the get data functions get data from a jsons and stores it in a vector so if the vector len () is greater tha zero it returns true  */

    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };
    //getting  data from json
    let get_json_words = p.get_data().unwrap();
    let get_json_info = p.playinfo().unwrap();
    //ensures that the length of data read from json is not empty data else it panickes
    assert!(!get_json_words.is_empty());
    assert!(!get_json_info.is_empty())
  }

  #[test]
  fn how_to_play() {
    /*
     *this function  tests if values returned from json  is greater than 0
     *it then  asserts that a welcome field is added if the user doesnt exist
     */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    let player_id = String::from(context.signer_account_id.clone());
    testing_env!(context);

    //creating a new user instance
    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };
    //calling the how_to_play function in  that returns a vector of  help info
    let helpinfo = p.how_to_play().unwrap();

    if p.userid == player_id {
      assert_eq!(helpinfo.len(), helpinfo.len() - 1);
    }

    assert!(!helpinfo.is_empty())
  }
  #[test]
  fn view_uncompleted_words() {
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    // let player_id = String::from(context.signer_account_id.clone());
    testing_env!(context);
    //creating a new user instance
    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };
    /*
    the view_uncompleted_words function returns a vector of hidden words where it is not empty
    */
    let uncompleted = p.view_uncompleted_words().unwrap();
    assert!(!uncompleted.is_empty());
  }

  #[test]
  fn test_random_word_is_picked_at_correct_index() {
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    let player_id = String::from(context.signer_account_id.clone());
    testing_env!(context);
    //creating a new user instance
    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };
    //this gets data at a certain index and  shows that it creats data from that index
    let index = 1;
    let random = p.random_word(index).unwrap();
    let data = p.get_data().unwrap()[index].clone();

    assert_eq!(random.meaning, data.meaning);
    /*the random word function hides the  word  from data but saves in the current user  guess  */
    if p.userid == player_id {
      assert_eq!(p.guess, data.word);
    }
  }

  #[test]
  fn test_if_the_userinput_solution_is_equal_saved_word() {
    /*this  */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    let player_id = String::from(context.signer_account_id.clone());
    testing_env!(context);
    //creating a new user instance
    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };
    let input_solution = "Near".to_string();
    p.random_word(0).unwrap();
    //checks the solution if is the same as the word and returns status message

    let result = p.check_solution(input_solution);
    if p.userid == player_id {
      assert!(result.is_ok());
    }
    //if saved word dosent exist it returns an err
    if p.userid != player_id {
      assert!(result.is_err());
    }
  }

  #[test]
  fn test_progress() {
    /*this tests if  the check progress returns  fuctions  checks the progress of the user figuring out the word generated  and returns an enum  depending on the current progress status   */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };

    //when turns are  above zero and word is not yet known returs  inprogress
    let mut turns = 6;
    let mut word = "near".to_string();
    let letters = Letter::get_letters(&mut word);
    let status = p.check_progress(turns, &letters);
    assert_eq!(Status::Inprogress, status);

    // when  all letters are  reveled returns a win
    //changing status to true
    turns = 4;
    let mut letters = Letter::get_letters(&mut word);
    for l in letters.iter_mut() {
      l.is_revealed = true;
    }
    let status = p.check_progress(turns, &letters);
    assert_eq!(Status::Completed, status);

    //when turns is 0 returns a fail
    turns = 0;
    let mut word = "near".to_string();
    let letters = Letter::get_letters(&mut word);
    let status = p.check_progress(turns, &letters);
    assert_eq!(Status::Failed, status);
  }

  #[test]
  fn test_view_completed() {
    /* generates  vector of compled words   */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let mut p = Player {
      userid: "onchez.testnet".to_string(),
      guess: String::from("Near"),
      word_progress: String::from("_ _ _"),
      completed: Vec::new(),
      turns: 0,
      is_revealed: false,
    };
    p.completed.push(CompletedWord {
      word: String::from("near"),
      status: String::from("completed"),
      trials_completed_at: String::from("near"),
    });
    p.completed.push(CompletedWord {
      word: String::from("near"),
      status: String::from("completed"),
      trials_completed_at: String::from("near"),
    });
    p.completed.push(CompletedWord {
      word: String::from("near"),
      status: String::from("completed"),
      trials_completed_at: String::from("completed"),
    });

    assert_eq!(p.completed.len(), 3)
  }
}
