## Run

This project is a command line interface for test driven workshop, originally build for [ifi-atelier-spark](https://github.com/lucas-dclrcq/ifi-atelier-spark).

## Compile

you need to install [rust](https://www.rust-lang.org/learn/get-started) to compile the project.

1. `git clone --recurse` this repository to get the java example project.
2. build the project with `cargo build`
3. run `./test_{command}.sh` to test available commands.
4. follow your progression [here](http://spark-leaderboard.hoohoot.org/)

## Available commands

| command        | description   | option
| -------------  |:-------------:| -----:|
| foreman init   | initialize the session
| --hard (skip the intro) |
| foreman next   | execute tests, perform git merge on success   | - |
| foreman --help | display the cli help | - |