## The `mu-ald` command

The `mu-ald` command calls the `sem-sfe-mu-ald` module. It produces a fixpoint
system and a list of symbolic $\exists$-moves from the given labelled
transition system, and $\mu$-calculus formula.

    sem-sfe-cli [OPTIONS] mu-ald <LTS_ALD> <MU_CALC_FORMULA> <STATE>

`<LTS_ALD>`

: A path to a file describing a labelled transition system in the Aldebaran format,
from the CADP toolset. The following link contains a description of the grammar:
<https://www.mcrl2.org/web/user_manual/tools/lts.html>.

`<MU_CALC_FORMULA>`

: A path to a file containing a $\mu$-calculus formula. The grammar is described
in section [Mu-calculus formulae].

`<STATE>`

: A string which represents a state. Since the Aldebaran specification uses
natural numbers as nodes' names, the state must be a number as
well. We want to verify whether if it satisfies the property described by
the $\mu$-calculus formula.

### Mu-calculus formulae

We want to parse the following syntax:

\begin{align*}
\varphi &::= tt  
            \mid ff
            \mid x  
            \mid \varphi\vee\varphi
            \mid \varphi \wedge \varphi
            \mid \mu x.\varphi
            \mid \nu x.\varphi
            \mid \langle a\rangle\varphi
            \mid [a]\varphi
\end{align*}

With $a\in Act$ and $x\in PVar$.
For the same reasons as in section [The `debug` command], we designed a grammar
that avoids, as much as possible, left recursion.
The following EBNF grammar describes a $\mu$-calculus formula.

\grammarindent1.3in
\begin{grammar}

<Label> ::= <Id> | `!' <Id> | `true'

<Atom> ::= `tt' | `ff' | `(' <Disjunction> `)'
\alt `<' <Label> `>' <Disjunction>
\alt `[' <Label> `]' <Disjunction>
\alt `mu' <Id> `.' <Disjunction>
\alt `nu' <Id> `.' <Disjunction>

<Conjunction> ::= <Atom> (`&&' <Atom>)*

<Disjuction>  ::= <Conjunction> (`||' <Conjunction>)*

<Id> ::= ( a C-style identifier )
\end{grammar}
