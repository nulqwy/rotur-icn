# rotur ICN resolver

"resolves" ICNs by: (non-exhaustive list)
- removing contextness relative to other commands (removes `c`, `w`, `move`, `back`)
- turning `cont`s into `line`s
- simplifying shapes where applicable (e.g. a `tri` with 2 same points is turned into a `line`)
