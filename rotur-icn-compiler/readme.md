# rotur ICN compiler

despite how simple ICN is, that's deceptive.

lowerer (AST -> HIR) and resolver (HIR -> LIR) are responsible for validating & simplifying ICN

- lowerer: validates arg type/count, cmd name
- resolver: simplifies commands, prepares them for feed into renderers, makes them contextless
