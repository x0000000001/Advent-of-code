# README

I used a modified Djisktra : since the "walls" (= winds) are moving over time, I characterize a node of the graph by : 

* position of the player
* time modulo the least common multiple (lcm) of the width and height

Indeed, after lcm(width, height) time has passed, the walls return to their original position.

For part 2, I used third criteria, an int, representing which step I am at. After a temporary goal is reached, I change the current goal.
