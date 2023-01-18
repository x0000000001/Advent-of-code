# README

## Monkeys, strange names and a linear solver

Part 1 was direct.

For part 2, I used a recursive representation of an equation.
I then constructed both sides of the final equation, along with a simplification function and a calculator solver.
This was made possible beacause all operations are fairly easy...

One thing that could be more optimized is when I check whether the unknown variable is in an equation. This is currently O(number of nodes) but could be O(ln(number of nodes)) (however the programm runs real fast).
