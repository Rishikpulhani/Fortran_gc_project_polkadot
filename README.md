# Tech GC Project

An innovative project combining a Python-based platformer game with Bitcoin testnet transaction capabilities implemented in Rust.

## Game Component

### Overview
A 2D platformer game featuring:
- Multiple levels with increasing difficulty
- Score tracking system
- Various enemy types (bees, slimes)
- Collectible diamonds
- Environmental hazards (water, lava)
- Moving platforms and bridges

### Requirements
- Python 3.7+
- Pygame library
```bash
pip install pygame
```

### Game Features
- **Level Editor**: Create and modify game levels
- **Dynamic Backgrounds**: Multiple background options
- **Sound Effects**: Background music and action sounds
- **Animation**: Animated player character and enemies
- **Save/Load System**: Progress saving functionality

### Controls
- **Movement**: Left/Right arrow keys
- **Jump**: Space bar or Up arrow key
- **Menu Navigation**: Mouse click

### Running the Game
```bash
# Run the main game
python3 main.py

# Run the level editor
python3 level_editor.py
```

## Blockchain Component

### Overview
A Rust-based Bitcoin testnet integration featuring:
- Wallet management
- Transaction creation
- Runes protocol support
- Multi-explorer transaction tracking

### Requirements
- Rust and Cargo
- Bitcoin Development Kit (BDK)
- Network connectivity for testnet

### Installation
```bash
# Install Rust dependencies
cargo build
```

### Key Features
1. **Wallet Operations**
   - Testnet wallet initialization
   - Address generation
   - Balance checking
   - Transaction signing

2. **Runes Integration**
   - Custom Runes payload creation
   - OP_RETURN script generation
   - Zero-value transaction support

3. **Transaction Management**
   - Fee rate configuration
   - PSBT (Partially Signed Bitcoin Transaction) handling
   - Transaction broadcasting

4. **Transaction Tracking**
   - Multiple explorer support:
     * Mempool.space
     * Blockstream.info
     * BlockCypher

### Usage
```bash
# Run the blockchain component
cargo run
```

### Transaction Structure
The program creates transactions with:
- OP_RETURN output containing Runes payload
- Default fee rate of 1.0 sat/vB
- Zero-value outputs for Runes protocol
- Testnet network compatibility

### Security Note
Replace the default wallet descriptor in the code with your actual descriptor:
```rust
"wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)"
```

## Development
- Game development: Python with Pygame
- Blockchain integration: Rust with BDK
- Network: Bitcoin Testnet



