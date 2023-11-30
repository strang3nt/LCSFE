## The `mu-ald` command

The `mu-ald` command calls the `lcsfe-mu-ald` module. It produces a fixpoint
system and a list of symbolic $\exists$-moves from the given labelled
transition system, and $\mu$-calculus formula.

    lcsfe-cli [OPTIONS] mu-ald <LTS_ALD> <MU_CALC_FORMULA> <STATE>

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

We want to parse the following syntax.

\begin{align*}
A &::= a \mid true \mid \neg a \\
\varphi &::=\bb{t}
            \mid\bb{f}
            \mid x  
            \mid \varphi\vee\varphi
            \mid \varphi \wedge \varphi
            \mid \mu x.\varphi
            \mid \nu x.\varphi
            \mid \langle A\rangle\varphi
            \mid [A]\varphi
\end{align*}

With $a\in Act$ and $x\in PVar$.
We designed a grammar that avoids, as much as possible, left recursion.
The following EBNF grammar describes a $\mu$-calculus formula.

\begin{align*}
\nonterminal{atom} \enspace &::= \enspace \terminal{tt} \mid\terminal{ff}
        \mid \terminal{(}\enspace\nonterminal{mu\_calc}\enspace\terminal{)}
        \mid \nonterminal{id} \\[2mm]
\nonterminal{modal\_op}\enspace &::= \enspace\terminal{<}\enspace\nonterminal{label}
        \enspace\terminal{>}\enspace\nonterminal{atom}
        \mid \enspace\terminal{[}\enspace\nonterminal{label}\enspace\terminal{]}
        \enspace\nonterminal{atom} \\[2mm]
\nonterminal{conjunction}\enspace &::= \enspace\nonterminal{modal\_op}\enspace
        (\terminal{\&\&}\enspace\nonterminal{modal\_op}) \\[2mm]
\nonterminal{disjunction}\enspace &::= \enspace\nonterminal{conjunction}\enspace
        (\terminal{||}\enspace\nonterminal{conjunction}) \\[2mm]
\nonterminal{fix\_op}\enspace &::= \enspace\terminal{mu}\enspace\nonterminal{id}
        \enspace\terminal{.}\enspace\nonterminal{disjunction}
        \mid \enspace\terminal{nu}\enspace\nonterminal{id}
        \enspace\terminal{.}\enspace\nonterminal{disjunction} \\[2mm]
\nonterminal{mu\_calc}\enspace &::= \enspace\nonterminal{fix\_op}
        \mid\nonterminal{disjunction} \\[2mm]
\nonterminal{label}\enspace &::= \enspace\terminal{true}\mid\nonterminal{id}
        \mid\terminal{!}\enspace\nonterminal{id}\\[2mm]
\nonterminal{id}\enspace &::= \enspace \texttt{"}\enspace
  (\mbox{ a C-style identifier }) \enspace \texttt{"}
\end{align*}

Moreover, we designed this grammar to respect some standard conventions:
modal operators $[a]$ and $\langle a\rangle$ bind stronger than $\vee, \wedge$,
and the fixpoint operators capture everything after the \terminal{.} character.
The consequence is that a formula
$\mu x( ([a] x) \vee\nu y(\langle a\rangle y\wedge\bb{f}))$ can be
written as $\mu x.[a] x\vee\nu y.\langle a\rangle y\wedge\bb{f}$, minimizing the
use of parenthesis.
Whenever we wish to add to a modal operator anything other than the syntactic
categories $\bb{t}$, $\bb{f}$ or $x\in PVar$, parenthesis must be used, this is
due to the inherent limitations of the type of parser we used. This is expressed
by the rule $\nonterminal{atom}$.
