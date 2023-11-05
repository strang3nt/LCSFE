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

<Atom> ::= `tt' | `ff' | `(' <MuCalc> `)'
        | <Id>

<ModalOp> ::= `<' <Label> `>' <Atom>
        | `[' <Label> `]' <Atom>
        | <Atom>

<Conjunction> ::= <ModalOp> (`&&' <ModalOp>)*

<Disjuction>  ::= <Conjunction> (`||' <Conjunction>)*

<Fix> ::= | `mu' <Id> `.' <Disjunction>
         | `nu' <Id> `.' <Disjunction>

<MuCalc> ::= <Fix> | <Disjunction>

<Label> ::= `true' | <Id>

<Id> ::= ( a C-style identifier )

\end{grammar}

Morover, we designed this grammar to respect some standard conventions:
the modal operators $\square$ and $\lozenge$ binds stronger than $\vee, \wedge$,
and the fixpoint operators, capture everything after the `.' character.

The consequence is that a formula $\mu x( (\square x) \vee\nu y(\lozenge y \wedge ff))$ can be
written as $\mu x.\square x\vee\nu y.\lozenge y\wedge ff$, minimizing the use of
parenthesis.
Whenever we wish to add to a modal operator anything different from the syntactic
categories $tt$, $ff$ or $x\in PVar$, parenthesis must be used, this is due to the
inherent limitations of the type of parser we used. This is expressed
by the rule `<Atom>`.
