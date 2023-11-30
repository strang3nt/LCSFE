# Structure of the project

The tool is divided into multiple modules, namely:

 - `lcsfe-algorithm`,
 - `lcsfe-cli`,
 - `lcsfe-common`,
 - `lcsfe-pg`,
 - `lcsfe-mu-ald`.

Module `lcsfe-algorithm` is the core of the project, it contains an implementation
of the local algorithm for verifying solutions of systems of fixpoint equations,
over complete lattices.
Module `lcsfe-cli` is a command line interface.
Modules `lcsfe-pg` and `lcsfe-mu-ald` both use the local algorithm.
Modules `lcsfe-pg` and `lcsfe-mu-ald` both use the local algorithm.
They take as input some specification language and some verification logic.
Then, they translate this input to a system of fixpoint equations, and generate
the correct symbolic $\exists$-moves for their respective operators,
after which they call the local algorithm to solve the verification problem,
and the output is passed to `lcsfe-cli` to be printed.
Module `lcsfe-common` exposes a common interface that
`lcsfe-pg` and `lcsfe-mu-ald` use to provide their results to the command line
interface module, it avoids circular dependency.

<!-- //www.plantuml.com/plantuml/png/ROy_2uCm4CNt-nH7fouHT2mYIYTdtQJ3eQSLygSawLJwtNjgnGubGnw_ztk1b26IZq-ZOVHa60CR5KR65tJVg4XFpi_nRcv80PiAkkR1FKPFDcYApc-yFHQzdbScDymsiX-fPpp94LYdFw8pnjbjV_sZPb1dgDHIDYtcIR8yUS4y7rZpH96B0kfqDIgGIj9PLYBlwQ3fJYMzeMGxoAy_ -->

\begin{figure}[H]
\captionsetup{justification=centering}
\centering
\includegraphics[width=0.5\textwidth]{img/lcsfe-diagram.png}
\caption{An informal component diagram of \texttt{LCSFE}.}
\label{img:design}
\end{figure}

Figure \ref{img:design} represents how the various modules of `LCSFE` are
related. In the diagram, Spec translator represents both `lcsfe-pg` and
`lcsfe-mu-ald`. From the diagram we understand that `lcsfe-algorithm` offers
an interface, represented by the ball notation, which is accessed by every other
module. The `lcsfe-common` crate exposes a trait, represented in the diagram
as an interface, via the ball notation.
The goal of this trait is to uniform the results computed by Spec translator,
so that `lcsfe-cli` can easily access and print them via the same
common interface.
Spec translator module is used by `lcsfe-cli`: the former
takes as input a specification file and some verification logic, and provides
to the latter the results of the computation.
