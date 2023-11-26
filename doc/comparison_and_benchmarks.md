# Examples and performance considerations

In this section we show two examples of executions of `sem-sfe`. We solve the
same two examples with the tools Oink [@oink] and mCRL2 [@mcrl2]. We use them
to to highlight our tool's performance. Every command is run on the same machine.
We are going to show an example with a parity game, and one using the
$\mu$-calculus.

We use the following parity game, the same as in in Figure \ref{fig:examplegame}.

```
parity 4;
0 6 1 4,2 "Africa";
4 7 1 0 "Antarctica";
1 5 1 2,3 "America";
3 6 0 4,2 "Australia";
2 8 0 3,1,0,4 "Asia";
```


We want to know whether player $\exists$ wins from node `Antarctica`, to do that
we run the following command.

    > cargo run -r -- pg tests/parity_games/test_03.gm Antarctica

File `test_03.gm` contains the game specification..
The command `cargo run -r` compiles and run `sem-sfe` in release mode, which creates
an optimised executable. The compilation
might take a few seconds. Then, aside from the compilation messages, the output
is the following.

    Preprocessing took: 0.000022963 sec.
    Solving the verification task took: 0.000010881 sec.
    Result: Player 1 wins from vertex Antarctica

Before running the local algorithm there is a preprocessing phase. The
preprocessing phase encompasses extracting the fixpoint system from the
specification, generating, and composing the symbolic $\exists$-moves.
In the case of parity games the preprocessing phase consists in extracting, the
system of fixpoint equations, and composing the moves. Symbolic $\exists$-moves
for conjunction and disjunction, the only operators appearing in this instance,
are provided by default. In the case of the `sem-sfe-mu-ald`, the preprocessing
phase is comprised of extracting the system of fixpoint equations from the
$\mu$-calculus formula, generating the symbolic moves for each operator, and
composing them into the symbolic $\exists$-moves for the system.

We solve the same parity game, on the same machine, with Oink, via the following
command.

    > oink tests/parity_games/test_03.gm -p

The output of the command is shown below.

    [    0.00] parity game with 5 nodes and 11 edges.
    [    0.00] parity game reindexed
    [    0.00] parity game renumbered (4 priorities)
    [    0.00] no self-loops removed.
    [    0.00] 2 trivial cycles removed.
    [    0.00] preprocessing took 0.000017 sec.
    [    0.00] solved by preprocessor.
    [    0.00] total solving time: 0.000033 sec.
    [    0.00] current memory usage: 4.62 MB
    [    0.00] peak memory usage: 4.75 MB
    [    0.00] won by even: America Asia Australia
    [    0.00] won by odd: Africa Antarctica

In this very simple example `sem-sfe` performance are on par with Oink's,
even thought Oink finds the global solution, instead of just a winner from a node.

We provide another example, this time we verify a property on a mCRL2 specification.
We use the "Gossips" example from the tutorial of mCRL2, which can be found
at the following link: <https://www.mcrl2.org/web/user_manual/tutorial/gossip/index.html>.
In order to use such specification in `sem-sfe`, we
convert it to a labelled transition system in the Aldebaran format,
using the tool `ltsconvert`, from mCRL2's toolset.
We want to check deadlock liveness.

    > cargo run -r -- mu-ald tests/example_mucalc/gossips.aut \
      tests/example_mucalc/deadlock-liveness 0

We pass as input to the command line interface the Aldebaran specification file,
`gossips.aut`, and the file containing the $\mu$-calculus formula. We want to
perform local model checking from state $0$ of the labelled transition system.

    Preprocessing took: 0.02513744 sec.
    Solving the verification task took: 0.000013575 sec.
    Result: The property is satisfied from state 0

To do the same with mCRL2, we run the following commands.
We first need to translate the mCRL2 specification the to `.lps`, which is a
file format used internally by mCRL2.

    > mcrl22lps gossip.mcrl2 gossip.lps

The command below takes a $\mu$-calculus formula, in `gossip.mcf`, and the file
we just generated `gossip.lps`, and builds a type of system of boolean fixpoint
equations, called parameterised boolean equation system.
This process took $0.014746$ seconds to finish.

    > lps2pbes --formula=gossip.mcf gossip.lps gossip.pbes --timings

Finally, we solve the model-checking task, via the following command.

    > pbes2bool -rjittyc gossip.pbes --timings

Below we show the output printed after the execution.

    true
        - tool: pbes2bool
          timing:
            instantiation: 1.397916
            solving: 0.002444
            total: 1.424587

Just as in the previous case, we remind that the solution provided by mCRL2 is global:
there is no deadlock configuration in the whole system. In `sem-sfe` we only
verify that there is no deadlock from state $0$. Here `sem-sfe`
shows measurably better performance.

# References