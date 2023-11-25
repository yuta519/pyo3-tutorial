## References
- [Official](https://www.maturin.rs/tutorial#program-the-guessing-game-in-rust)

## How to try
- Compile
```bash
maturin develop
```

- Run compiled code from python
```bash
python

$ python
Python 3.10.6 (main, Jan  9 2023, 02:24:36) [Clang 14.0.0 (clang-1400.0.29.102)] on darwin
Type "help", "copyright", "credits" or "license" for more information.

>>> import guessing_game
>>> gussing_game
>>> guessing_game.guess_the_number()
Guess the number!
Please input your guess!
23
You guessed: 23
Too small!
Please input your guess!
75
You guessed: 75
Too big!
Please input your guess!
53
You guessed: 53
Too big!
Please input your guess!
35
You guessed: 35
Too small!
Please input your guess!
40
You guessed: 40
Too big!
Please input your guess!
37
You guessed: 37
Too small!
Please input your guess!
39
You guessed: 39
You win!
>>>
```
