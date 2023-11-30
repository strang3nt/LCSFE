# Installation

You should first have a working installation of Rust and Cargo, 1.73 and above.
This project has not been tested with versions of Rust below 1.73.

To compile this project download it from this repository, and run
`cargo build -r` from the terminal emulator. The compiled
executable should be located in `lcsfe/target/release`.

# Usage

This application is a command line interface.
An invocation of `LCSFE` looks like this:

    lcsfe-cli [OPTIONS] <COMMAND>

where `[OPTION]` is a list of flags and `<COMMAND>` is the name of the type of
input we are going to feed to the tool.

There are 2 possible options, which can be enabled:

-n or --normalize

: If enabled, the underlying system of fixpoint equations is normalized during
the preprocessing phase.

-e or --explain

: A flag that makes the program print useful information to stdout: the underlying
system of fixpoint equations, and the composed symbolic $\exists$-moves.

A `<COMMAND>` string is one of the following: `debug`, `pg`, `mu-ald`, followed
by their respective inputs. We are going to introduce these commands in the
next sections.

## The `debug` command

The debug command has the following structure:

    lcsfe-cli [OPTIONS] debug <ARITY>\
    <FIX_SYSTEM> <BASIS> <MOVES_SYSTEM> <ELEMENT_OF_BASIS> <INDEX>

`<ARITY>`

: A path to a file containing definitions of functions. The file must be
formatted as follows: each line contains a string of characters and a
natural number. The string represents the name of a function, which
is going to be used in the system of fixpoint equations. The natural number
represents the arity of the function. The names `and` and `or` can be
declared, but will be ignored.

`<FIX_SYSTEM>`

: A path to a file containing the definition of a system of fixed
point equations. A function must be an either an and or or function,
or it must be specified in the arity file. We give a precise
grammar specification in section [Input grammar specification].

`<BASIS>`

: A path to a file containing all the elements of the basis. Each new line
must contain a string, which is an element of the basis.

`<MOVES_SYSTEM>`

: A path to a file containing the symbolic $\exists$-moves for the
system of fixpoint equations. There must be a symbolic $\exists$-move for
all possible combinations of functions introduced in the file, and basis
elements introduced in the file. We give the grammar specification in section
[Input grammar specification].

`<ELEMENT_OF_BASIS>`

: The element of the basis which we want to verify is
part of the solution of the system of fixpoint equations.

`<INDEX>`

: A number representing the equation, and thus the variable which
we want to check is above, with respect to some ordering, the basis element.

### Input grammar specification

We now give the grammar, in EBNF form, for systems of fixpoint
equations, symbolic $\exists$-moves, a basis and the arity specification.

\begin{align*}
\nonterminal{eq\_list}\enspace &::= \enspace\nonterminal{eq}\enspace\nonterminal{eq\_list}
  \enspace\terminal{;}\mid\nonterminal{eq}\enspace\terminal{;}\\[2mm]
\nonterminal{eq}\enspace &::=\enspace\nonterminal{id}\enspace\terminal{=max}\enspace
  \nonterminal{or\_exp\_eq}\mid\nonterminal{id}\enspace\terminal{=min}\enspace\nonterminal{or\_exp\_eq}\\[2mm]
\nonterminal{atom}\enspace &::=\enspace\nonterminal{id}
  \mid\terminal{(}\enspace\nonterminal{or\_exp\_eq}\enspace\terminal{)}
  \mid\nonterminal{custom\_exp\_eq}\\[2mm]
\nonterminal{and\_exp\_eq}\enspace &::= \enspace\nonterminal{atom}\enspace
  (\terminal{and}\enspace\nonterminal{atom})^*\\[2mm]
\nonterminal{or\_exp\_eq}\enspace &::= \enspace\nonterminal{and\_exp\_eq}\enspace
  (\terminal{or}\enspace\nonterminal{and\_exp\_eq})^*\\[2mm]
\nonterminal{custom\_exp\_eq}\enspace &::= \enspace\nonterminal{op}\enspace
  \terminal{(}\enspace\nonterminal{or\_exp\_eq}\enspace(\terminal{,}\enspace
  \nonterminal{or\_exp\_eq})^*\enspace\terminal{)}\\[2mm]
\nonterminal{id}\enspace &::= \enspace \texttt{"}\enspace
  (\mbox{ a C-style identifier }) \enspace \texttt{"}\\[2mm]
\nonterminal{op}\enspace &::= \enspace \texttt{"}\enspace
  (\mbox{ any ASCII string }) \enspace \texttt{"}
\end{align*}

The grammar above represents a system of fixpoint equations.
Notice that the syntactic category $and\_exp\_eq$ has a higher precedence than
$or\_exp\_eq$, this way we enforce the precedence of the operator $\wedge$ over $\vee$.
Tokens $id$ and $op$ are strings, the latter represents the name of an operator
provided by the user. If the goal is to parse $\mu$-calculus formulae, $op$ would
accept for example strings such as "diamond", or "box".
Note that all operators are expressed in terms of a function, except for
\terminal{and} and \terminal{or}, which are conveniently already provided, and
are infix. A C-style identifier respects the following regex pattern
`[a-zA-Z_][a-zA-Z0-9_]*`.

\begin{align*}
\nonterminal{sym\_mov\_list}\enspace &::= \enspace\nonterminal{sym\_mov\_eq}\enspace\nonterminal{sym\_mov\_list}
  \enspace\terminal{;}\mid\nonterminal{sym\_mov\_eq}\enspace\terminal{;}\\[2mm]
\nonterminal{sym\_mov\_eq}\enspace &::= \enspace\terminal{phi}\enspace\terminal{(}
  \enspace\nonterminal{id}\enspace\terminal{)}
  \enspace\terminal{(}\enspace\nonterminal{num}\enspace\terminal{)}
  \enspace\terminal{=}\enspace\nonterminal{disjunction}\\[2mm]
\nonterminal{conjunction}\enspace &::= \enspace\nonterminal{atom}\enspace
  (\terminal{and}\enspace\nonterminal{atom})^*\\[2mm]
\nonterminal{disjunction}\enspace &::= \enspace\nonterminal{conjunction}\enspace
  (\terminal{or}\enspace\nonterminal{conjunction})^*\\[2mm]
\nonterminal{atom}\enspace &::= \enspace\terminal{[}\enspace\nonterminal{id}
  \enspace\terminal{,}\enspace\nonterminal{num}\enspace\terminal{]}
  \mid\terminal{true}\mid\terminal{false}\mid\terminal{(}
  \enspace\nonterminal{disjunction}\enspace\terminal{)}\\[2mm]
\nonterminal{id}\enspace &::= \enspace \texttt{"}\enspace
  (\mbox{ a C-style identifier }) \enspace \texttt{"}\\[2mm]
\nonterminal{num}\enspace &::= \enspace\Nat
\end{align*}

The grammar above represents the symbolic $\exists$ moves for some operators.
Note that, similarly to what we did for the grammar of systems of fixpoint
equations, the conjunction operator has a greater precedence than the
disjunction operator.

We now give the grammar of a basis: it is simply a list of strings, separated
by the new-line character `\n`.

\begin{align*}
\nonterminal{basis} \enspace &::=
    \enspace\nonterminal{basis\_elem}\enspace\terminal{\textbackslash n}\enspace\nonterminal{basis}
    \mid\nonterminal{basis\_elem}\\[2mm]
\nonterminal{basis\_elem} \enspace &::= \enspace\enspace \texttt{"}\enspace
  (\mbox{ any ASCII string }) \enspace \texttt{"}
\end{align*}

Follows the grammar specification of a file containing the name of the operators
and their arity.

\begin{align*}
\nonterminal{arity} \enspace &::= \enspace\nonterminal{op\_name}\enspace\nonterminal{num}
    \enspace\terminal{\textbackslash n}\enspace\nonterminal{arity}
    \mid\nonterminal{op\_name}\enspace\nonterminal{num}\\[2mm]
\nonterminal{op\_name}\enspace &::= \enspace \texttt{"}\enspace
  (\mbox{ a C-style identifier }) \enspace \texttt{"}\\[2mm]
\nonterminal{num}\enspace &::= \enspace\Nat
\end{align*}