# Tech GC Project

This project combines a Python-based game with blockchain transaction capabilities using Rust.

## Game Component

### Requirements

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install required packages:
* Pygame

```bash
pip install pygame
```

### Game Usage

Run the game with:
```bash
python3 main.py
```

#### Game Controls
* Use Space or Up arrow key to jump
* Use Left & Right arrow keys to move
* Collect diamonds while avoiding bees, slimes, water & lava
* Clear all levels to win

## Blockchain Component

### Requirements

The blockchain component requires:
* Rust and Cargo
* Bitcoin Development Kit (BDK)

### Transaction Features

The Rust component provides:
* Wallet initialization with testnet support
* Balance checking
* OP_RETURN transaction creation with Runes payload
* Transaction broadcasting
* Explorer links to track transactions on:
  - Mempool
  - Blockstream
  - BlockCypher

### Building and Running

```bash
cargo build
cargo run
```



