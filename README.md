# NEAR Wordchaser Smart Contract

This is game smart contract to test your english knowledge to know words that lets you choose a word then gives an exmaple and meaning you can give a solution from the example and meaning given

## Tools Required

Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install [Rust](https://www.rust-lang.org/tools/install):

## file Layout

```
Root Folder
├── contract
│   ├── build.sh
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
|   |   ├── data.rs
|   |   ├── how_play.rs
|   |   └── lib.rs
│   └── test.sh
└── README.md

```

## Game Function Calls

To start the game

```
near call word.onchez.testnet  how_to_play --accountId   Your accountId
```

To get a random word

```
near call word.onchez.testnet  random_word --accountId   Your accountId
```

To view completed words

```
near call word.onchez.testnet  view_available_words --accountId   Your accountId
```

To check solution

```
near call word.onchez.testnet  check_solution '{"word":"you_solution"}'--accountId   Your accountId
```

To add more tunrs in the game charge 1near per 10 turns added

```
near call word.onchez.testnet   add_more_turns --accountId   Your accountId
```

#### Author

- onchez brian <brianonchezz@gmail.com> [@onchez2](https://twitter.com/onchez2)

link to video : 
