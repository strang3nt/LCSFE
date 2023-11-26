## The `pg` command

The `pg` command uses the `sem-sfe-pg` module, to build a system of fixed point
equations and the symbolic $\exists$-moves from a parity game, and verify whether
if the given node is winning for player $\exists$ (or player 0, or player Even).

This is a typical command for the `pg` command:

    sem-sfe-cli [OPTIONS] pg <GAME_PATH> <NODE>

`<GAME_PATH>`

: A path to a file containing a PGSolver file specification.

`<NODE>`

: A string which must refer to the name of the node, if specified in the input file,
or to the id of a node.
