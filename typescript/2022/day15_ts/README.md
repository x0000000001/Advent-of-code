# README

Part 1 : intervalls intersection, and recursively counting the intersection count of a list of intervalls.

I struggled a bit too solve part 2.
I started trying to count 2d intervalls superposition, but it appeared to not be as simple as the 1d case.
The brute-force method was obviously too slow for part 2.

I ended up with a **divide and conquer** algorithm : starting from a square that represents the whole map (min, max x and min, max y), we do for each square :

* if the square if entirely covered by one of the beacon range, we drop it
* else, we divide it in four and add them to the queue
* if the square is 1x1, it's the solution
