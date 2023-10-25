# Installation

You should first have a working installation of Rust and Cargo, 1.73 and above. This project has not been tested with versions of Rust below 1.73.

Download this project, <https://github.com/strang3nt/sem-lmc.git>, go into
its root folder and execute `cargo run -r` from terminal. The compiled
executable should be located in `sem-lmc/target/release`.

# Usage

The compiled binary is a command line interface. In the following we list and
provide an explanation for all the commands and options.
The following listing is a correct command line invocation:

    sem-lmc-cli [OPTIONS] <COMMAND>

where `[OPTION]` is a list of flags and `<COMMAND>` is the name of the type of
input we are going to feed to the tool.

Options:

-n or --normalize

:  If enabled, the underlying system of fixpoint equations is normalized during the preprocessing phase.

-e or --explain

: A flag that makes the program print useful information to stdout: the underlying system of fixpoint equations, and the symbolic existential-moves, before and after composition.

Commands:

debug <ARITY> <FIX_SYSTEM> <BASIS> <MOVES_SYSTEM> <ELEMENT_OF_BASIS> <INDEX>

:

pg <GAME_PATH> <NODE>

:

mu-ald <LTS_ALD> <MU_FORMULA>  

:

help <COMMAND>

: Prints information about the tool, or the help of a command, if given.

## Tutorial

This is a brief tutorial that provides a few examples for all types commands.
We suppose to be in the terminal emulator, in the following location: `sem-lmc-cli/target/release`.
The project should be already compiled for release.
The repository contains the files we are going to use, under the folder `sem-lmc-cli/tests`.

### Parity games

The command:

    ./sem-lmc-cli pg -g ../../tests/parity_games/test_03.gm -n Antarctica

will parse the the file below, in PGSolver format:

```
parity 4;
0 6 1 4,2 "Africa";
4 7 1 0 "Antarctica";
1 5 1 2,3 "America";
3 6 0 4,2 "Australia";
2 8 0 3,1,0,4 "Asia";
```

and ask whether if the existential player can win from vertex `Antarctica`.

### $\mu$-calculus
