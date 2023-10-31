# Installation

You should first have a working installation of Rust and Cargo, 1.73 and above. This project has not been tested with versions of Rust below 1.73.

To compile this project simply download it from the repository, and run
`cargo build -r` from the terminal emulator. The compiled
executable should be located in `sem-sfe/target/release`.

# Usage

This application is a command line interface.
An invocation of `sem-sfe` looks like this:

    sem-sfe-cli [OPTIONS] <COMMAND>

where `[OPTION]` is a list of flags and `<COMMAND>` is the name of the type of
input we are going to feed to the tool.

There are 2 possible options, which can be enabled:

-n or --normalize

: If enabled, the underlying system of fixpoint equations is normalized during
the preprocessing phase.

-e or --explain

: A flag that makes the program print useful information to stdout: the underlying
system of fixpoint equations, and the symbolic existential-moves..

A `<COMMAND>` string is one of the following: `debug`, `pg`, `mu-ald`, followed
by their respective inputs. We are going to introduce all these commands in the
next sections.

## The `debug` command

The debug command has the following structure:

    sem-sfe-cli [OPTIONS] debug <ARITY>\
    <FIX_SYSTEM> <BASIS> <MOVES_SYSTEM> <ELEMENT_OF_BASIS> <INDEX>

`<ARITY>`

: A path to a file containing definitions of functions. The file must be
formatted as follows: each line contains a string of characters and an
integer number. The string represents the name of a function, which
is going to be used in the system of fixpoint equations. The integer
represents the arity of the function. The names and and or can be
declared, but will be ignored.

`<FIX_SYSTEM>`

: A path to a file containing the definition of a system of fixed
point equations. A function must be an either an and or or function,
or it must be specified in the arity file. We are going to give a precise
grammar specification in section [Input grammar specification].

`<BASIS>`

: A path to a file containing all the elements of the basis. Each line
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
is going to be substituted by the element of the basis specified.

### Input grammar specification

We now give the grammar, in EBNF form, for systems of fixpoint
equations and symbolic $\exists$-moves.

\grammarindent1.4in
\begin{grammar}

<EqList> ::= <Eq> <EqList> `;' | <Eq> `;'

<Eq> ::= <Id> `=max' <OrExpEq> | <Id> `=min' <OrExpEq>

<Atom> ::= <Id> | `(' <OrExpEq> `)' | <CustomExpEq>

<AndExpEq> ::= <Atom> (`and' <Atom>)*

<OrExpEq> ::= <AndExpEq> ( `or' <AndExpEq> )*

<CustomExpEq> ::= <Op> `(' <OrExpEq> (`,' <OrExpEq>)* `)'

<Id> ::= ( a C-style identifier )

<Op> ::= ( any ASCII string )

\end{grammar}

The grammar above represents a system of fixpoint equations.
Notice that the syntactic category $AndExpEq$ has a higher precedence than
$OrExpEq$, this way we enforce the precedence of the operator and over or.
Tokens $Id$ and $Op$ are strings, the latter represents the name of an operator
provided by the user. If the goal is to parse $\mu$-calculus formulae, a possible
definition for OP would be $Op \in\{diamond,box\}$.

The following EBNF grammar describes a list of symbolic $\exists$-moves:
\grammarindent1.3in
\begin{grammar}

<SymMovList>  ::= <SymMovEq> <SymMovList> `;' | <SymMovEq> `;'

<SymMovEq>    ::= `phi' `(' <Id> `)' `(' <Num> `)' `=' <Disjunction>

<Conjunction> ::= <Atom> (`and' <Atom>)*

<Disjunction> ::= <Conjunction> (`or' <Conjunction>)*

<Atom>        ::= `[' <Id> `,' <Num> `]' | `true' | `false'
\alt `(' <Disjunction> `)'

<Id>          ::= ( a C-style identifier )

<Num>         ::= $\mathbb{N}$

\end{grammar}

To parse both grammars we used a parser libray called [Chumsky](https://github.com/zesterer/chumsky).
Chumsky is based on parser combinators, which is a parsing technique that allows
for easy to mantain code, and unlike parser generators, no unnecessary boilerplate.
The downside of parser combinator is that they usually have a limited support for
left recursion, which is why both grammars were built to avoid left-recursion.
Indirect left recursion is permitted, but in a limited way.
