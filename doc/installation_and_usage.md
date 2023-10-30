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

## Debug command

The debug has the following structure:

    sem-sfe-cli [OPTIONS] debug <ARITY>\
    <FIX_SYSTEM> <BASIS> <MOVES_SYSTEM> <ELEMENT_OF_BASIS> <INDEX>

`<ARITY>`

:A path to a file containing definitions of functions. The file must be
formatted as follows: each line contains a string of characters and an
integer number. The string represents the name of a function, which
is going to be used in the system of fixpoint equations. The integer
represents the arity of the function. The names and and or can be
declared, but will be ignored.

`<FIX_SYSTEM>`

: A path to a file containing the definition of a system of fixed
point equations. A function must be an either an and or or function,
or it must be specified in the arity file. We are going to give a precise
grammar specification in Input file grammar specification.

`<BASIS>`

: A path to a file containing all the elements of the basis. Each line
must contain a string, which is an element of the basis.

`<MOVES_SYSTEM>`

: A path to a file containing the symbolic $\exists$-moves for the
system of fixpoint equations. There must be a symbolic $\exists$-move for
all possible combinations of functions introduced in the file, and basis
elements introduced in the file. We give the grammar specification in
Input file grammar specification.

`<ELEMENT_OF_BASIS>`

: The element of the basis which we want to verify is
part of the solution of the system of fixpoint equations.

`<INDEX>`

: A number representing the equation, and thus the variable which
is going to be substituted by the element of the basis specified.
2.1.1 Input file grammar specification
In the following we give the grammar, in EBNF form, for systems of fixpoint
equations and symbolic $\exists$-moves.

The following EBNF grammar describes a list of symbolic $\exists$-moves:

\begin{grammar}

<EqList> ::= <Eq> <EqList> `;' | <Eq> `;'

<Eq> ::= <Id> `=max' <ExpEq> | <Id> `=min' <ExpEq>

<ExpEq> ::= <OrExpEq>

<Atom> ::= <Id> | `(' <ExpEq> `)' | <CustomExpEq>

<AndExpEq> ::= <Atom> (`and' <Atom>)*

<OrExpEq> ::= <AndExpEq> ( `or' <AndExpEq> )*

<CustomExpEq> ::= <Op> `(' <ExpEq> (`,' <ExpEq>)* `)'

<Id> ::= ( a C-style identifier )

<Op> ::= ( any ASCII string )

\end{grammar}

Notice that the syntactic category $AndExpEq$ has a higher precedence than
$OrExpEq$, this way we enforce the precedence of the operator and over or.
Tokens $Id$ and $Op$ are strings, the latter represents the name of an operator
provided by the user. If the goal is to parse $\mu$-calculus formulae, a possible
definition for OP would be $Op \in\{diamond,box\}$.
