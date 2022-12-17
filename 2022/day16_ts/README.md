# DAY 16

Very hard day in my opinion.

Current problem with my solution : doesn't work on the examples (but gives me the good results on my input !). \
I still have to figure out why.

Principle : depth first search with memoization on graph with nodes determined by (position, time).

For part 2: added boolean to know which player was playing.
Here is the trick : the 2 players don't need to be simulated both at the same time !
Run the player, then the elephant on the set of remaining unopened valves.

My current solutions run in < 50ms, which I am very proud of.

(However, I repeat that **these don't work on the given examples**).
