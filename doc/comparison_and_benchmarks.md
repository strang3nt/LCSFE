## Examples and performance considerations

In this section we show three examples of executions of `LCSFE`. We solve the
same three examples with Oink and mCRL2. We use them
to discuss the performance of our tool. All tests are performed on the same machine:
a laptop powered by an AMD Ryzen 5 5500 processor and $8$ gigabytes of RAM, with
the Linux kernel $6.5.11$.

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
The command `cargo run -r` compiles and run `LCSFE` in release mode, which creates
an optimised executable. The compilation
might take a few seconds. Then, aside from the compilation messages, the output
is the following.

    Preprocessing took: 0.000022963 sec.
    Solving the verification task took: 0.000010881 sec.
    Result: Player 1 wins from vertex Antarctica

Before running the local algorithm, a preprocessing phase takes place. The
preprocessing phase encompasses extracting the fixpoint system from the
specification, generating, and composing the symbolic $\exists$-moves.
In the case of parity games the preprocessing phase consists in extracting the
system of boolean fixpoint equations and composing the moves, there are only two
operators in a system of boolean fixpoint equations, $\wedge$, $\vee$ and we provide
symbolic $\exists$-moves for both by default.
In the case of `lcsfe-mu-ald` the preprocessing
phase is comprised of extracting the system of fixpoint equations from the
$\mu$-calculus formula, generating the symbolic moves for each operator appearing
in the formula, instantiated to the labelled transition system provided as input,
and composing them to symbolic $\exists$-moves.

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

In this very simple example `LCSFE` performance are on par with Oink's,
even thought Oink finds the global solution, instead of just a winner from a node.

The next two examples come from the mCRL2 tutorial, which can be found at the
the following link: <https://www.mcrl2.org/web/user_manual/tutorial>.
We want to solve two problems.
The first problem we want to solve is "The Rope Bridge": we want to know wheter
four adventurers can cross a bridge in 17 minutes.
We have the following constraints: no more than two persons can cross the bridge
at once, a flashlight needs to be carried by one of them every crossing. They
have only one flashlight. The four adventurers are not all equally skilled:
crossing the bridge takes them 1, 2, 5 and 10 minutes, respectively. A pair of
adventurers cross the bridge in an amount of time equal to that of the slowest
of the two adventurers. We define the following $\mu$-calculus formula:
$\mu X. \bb{t}\vee\langle true\rangle(\langle report(17) \rangle x)$.

In order to use a mCRL2 specification in `LCSFE`, we
convert it to a labelled transition system in Aldebaran format,
using the tool `ltsconvert`, from mCRL2's toolset. Then we run our tool.
The resulting transition system has 102 states, and 177 transitions.

    > cargo run -r -- mu-ald tests/example_mucalc/bridge-referee.aut \
      tests/example_mucalc/receive-17 0

We pass as input to the command line interface the Aldebaran specification file,
`bridge-referee.aut`, and the file containing the $\mu$-calculus formula. We want to
perform local model checking from state $0$ of the labelled transition system.
Follows the output of the command.

    Preprocessing took: 0.02513744 sec.
    Solving the verification task took: 0.000013575 sec.
    Result: The property is satisfied from state 0

To do the same with mCRL2, we run the following commands.
We first need to translate the mCRL2 specification the to `.lps`, which is a
file format used internally by mCRL2.

    > mcrl22lps bridge-referee.mcrl2 bridge.lps

The command below takes a $\mu$-calculus formula, in `formula_A-final.mcf`,
and the file
we just generated `bridge.lps`, and builds a kind of system of boolean fixpoint
equations, called parameterised boolean equation system.
This process took $0.017395$ seconds to finish.

    > lps2pbes --formula=formula_A-final.mcf bridge.lps bridge.pbes --timings

Finally, we solve the model checking task, via the following command.

    > pbes2bool -rjittyc bridge.pbes --timings

The output is the following.

    true
    - tool: pbes2bool
      timing:
        instantiation: 0.009156
        solving: 0.000032
        total: 0.038423

Notice how `LCSFE` performs roughly the same as mCRL2.

We now describe the next specification: "Gossips".
There are five agents, each have an information that must be shared to all of
them. Agents share information by making phone calls, and each time they share
all of their secrets.
We want to check whether this system is deadlock free. To do so, we use the following
$\mu$-calculus formula: $\varphi = \nu X . \Diamond tt\wedge \Box X$.
The result of the conversion of the mCRL2 specification to Aldebaran format is
a labelled transition system with 9152 states and 183041 transitions.

    > cargo run -r -- mu-ald tests/example_mucalc/gossips.aut \
      tests/example_mucalc/deadlock-liveness 0

We pass as input to the command line interface the Aldebaran specification file,
`gossips.aut`, and the file containing the $\mu$-calculus formula. We want to
perform local model checking from state $0$ of the labelled transition system.

    Preprocessing took: 0.16171992 sec.
    Solving the verification task took: 5.695183 sec.
    Result: The property is satisfied from state 0

We run the same commands as before, until we reach the following output.

    true
        - tool: pbes2bool
          timing:
            instantiation: 1.397916
            solving: 0.002444
            total: 1.424587

We see that in this case mCRL2 is much faster. Moreover, during the execution of
`LCSFE` we experienced a stack overflow: due to the recursive nature of the
local algorithm we employ, the recursive calls filled a stack of 8 megabytes of
size. In order to solve this verification stack, we incremented the size of the
stack.
