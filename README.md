# 🎮 minigames

A collection of terminal-based classic games written in Rust.
Currently includes:

- Hangman
- Tic Tac Toe

More games may be added in the future.

---

### How to Play

```bash
git clone https://github.com/your-username/minigames.git
cd minigames
cargo run
```

---

### 🪓 Hangman

- Guess the full word at once.
- Correctly guessed letters will be revealed.
- You get limited tries to guess the word correctly.
- Optional: to enable debug mode (show the secret word), run:

```bash
export DEBUG=true
```

This will only affect the current terminal session.

### 🤞Tic Tac Toe
- Classic 3x3 grid.
- Two-player local multiplayer.
- Enter positions using keys 1 through 9 (like a numpad).




### Development Status

Still a WIP. Core gameplay for each included game is functional.
Planned improvements include better UX, error handling, and more games.
