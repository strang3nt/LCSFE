## Tutorial

This is a brief tutorial that provides a few examples.
In the following we suppose to be in the terminal emulator, in the path: `sem-sfe-cli/target/release`.
The project should be already compiled for release.
The repository contains the files we are going to use, under the folder `sem-sfe-cli/tests`.

### Parity games

The command:

    ./sem-sfe-cli pg -g ../../tests/parity_games/test_03.gm -n Antarctica

will parse the file below, in PGSolver format:

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
