# Guess The Number Game
This is a simple command-line game written in Rust. The game generates a random number between 1 and 100, and your task is to guess it.

## How to Play
When you run the game, it will prompt you to enter a number. After you make a guess, the game will give you a hint whether your guess is too small, too big, or correct.

## Code Overview
The code uses the `rand` crate to generate a random number and the `std::cmp::Ordering` enum for comparing the guessed number with the secret number.

The main loop of the game is in the main function. It first generates a secret number, then enters a loop where it reads a number from the user, compares it to the secret number, and gives a hint to the user. If the guess is correct, it congratulates the user and breaks the loop, ending the game.

## Dependencies
- `rand` crate for generating random numbers

Happy guessing!