# The design of `sem-sfe`

## High-level structure

The tool is divided into multiple modules, namely:

 1. `sem-sfe-algorithm`,
 2. `sem-sfe-cli`,
 3. `sem-sfe-pg`,
 4. `sem-sfe-mu-ald`.

Module 1. is the core of the project, it contains the local algorithm for verifying
solutions for systems of fixpoint equations. Module 2. is the command line interface.
Modules 3. and 4. are interfaces to the local algorithm, they take as input some
specification language and some verification logic, and they translate such input
in a system of fixpoint equations and generate the symbolic $\exists$-moves.
<!-- www.plantuml.com/plantuml/png/NL4zRzj03DtvAuXC7Ie1MWiZID6Xo930WBq57LY9fnny7zSZPTWj-jzBoKeJ6ngEUE_naOzw4AMgKmpGvrXougGe71jnZ7y0IwwAT_QHOEa0tqLtMwfUppXIv8NUVh-Yd_AHvGA8rrmNpTEtMmlXUkc-8fvpZHJyxvLyXhzGEcmQ6x8MvhGYbzSqBxl2FMgb985iue-vTRU7bpLFqmU_OFQ2JyhLKSzlwHuxW9ILCR1Jp6u6lYPyr-ahAVB9uh10P7tDWw2hEMAX_a0Zozm5wx0ME0qu6zxOsa9xEfYOnCpBf4ZeV62BdvHyycaq1CIoUETxKf_JPoGpanmPZrp_mw0aXnQtDH_zC3kYd4KvsXXm_iG3bcpx8_-Q5hsiFkVuCmVV0v-yg3Hn_bCVzztjSTEsyvc-u6OpSE2HbE_luN6vTEa0ZBFoant9r6_LWrbCCVm3 -->

![A diagram that represents the design of `sem-sfe`\label{img:design}](http://www.plantuml.com/plantuml/png/NL4zRzj03DtvAuXC7Ie1MWiZID6Xo930WBq57LY9fnny7zSZPTWj-jzBoKeJ6ngEUE_naOzw4AMgKmpGvrXougGe71jnZ7y0IwwAT_QHOEa0tqLtMwfUppXIv8NUVh-Yd_AHvGA8rrmNpTEtMmlXUkc-8fvpZHJyxvLyXhzGEcmQ6x8MvhGYbzSqBxl2FMgb985iue-vTRU7bpLFqmU_OFQ2JyhLKSzlwHuxW9ILCR1Jp6u6lYPyr-ahAVB9uh10P7tDWw2hEMAX_a0Zozm5wx0ME0qu6zxOsa9xEfYOnCpBf4ZeV62BdvHyycaq1CIoUETxKf_JPoGpanmPZrp_mw0aXnQtDH_zC3kYd4KvsXXm_iG3bcpx8_-Q5hsiFkVuCmVV0v-yg3Hn_bCVzztjSTEsyvc-u6OpSE2HbE_luN6vTEa0ZBFoant9r6_LWrbCCVm3)

The diagram \ref{img:design} represents how the various modules of `sem-sfe` are
related. In the diagram, "Spec translator" represents both `sem-sfe-pg` and
`sem-sfe-mu-ald`. From the diagram we understand that `sem-sfe-algorithm` offers
an interface, represented by the ball notation, which is accessed by every other
module. The Spec translator module is used by `sem-sfe-cli`: the former is able to
take as input a specification file and some verification logic, and provides
to the latter a system of fixpoint equations and the symbolic $\exists$-moves, making
it possible to run the verification task via the local algorithm.

## The local algorithm module

<!-- the interface (normalization, composition of moves) the algorithm briefly -->

## How to contribute

<!-- the trait that must be implemented by each module -->
