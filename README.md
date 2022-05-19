# NEAR Wordchaser Smart Contract

This is game smart contract  
 to test your english knowledge to know words that lets you choose a word then gives an exaple and an explanation and you can give a solution from the example and meaning given

## Prerequisites

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

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Install dependencies

```
npm install
```

To start the game

```
near call app.onchez.testnet  how_to_play --accountId   Your accountId
```

To get a random word

```
near call app.onchez.testnet  random_word --accountId   Your accountId
```

#### Author

- onchez brian <brianonchezz@gmail.com> [@onchez](https://twitter.com/onchezz_2)
