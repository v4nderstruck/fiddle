# Semantics

**Why?** Need to convert any expressions into the standard form to solve using tableau
method.


## Standard Form

- maximization problem
- positivity constraint for each variable (0 as lower bound)
- all other constraints are equality constraints, rhs is non-negative

## Conversion

- **maximization** convert minimization objective `f` to `-f`
- **inequality constraints** convert to equality constraints by adding slack
  variables `f <= n` to `f + s = n` where `s >= 0` and `f >= n` to `f - s = n`
  where `s >= 0`
- **variable not bounded to 0** convert variables `x >= n` with `n != 0`  to equations
`x = x' + n` and `x' = x - n` with `x' >= 0`. 
- any variable `z` that are not restricted by a bound can be replaced by the difference
of two non-negative variables `z = z'' - z'` where `z' >= 0` and `z'' >= 0` . 
