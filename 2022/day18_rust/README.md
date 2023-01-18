# README

Part 1 : number of cubes * 6 - number of neighbour nodes * 2.

Part 2 : \
This problem is highly simplified by the fact that the coordinates range are low, so we can loop through the entirety of the considered 3d space. \
I used a queue to go through all nodes that are considered to be the outside class (by first inserting the corners of the space) and keep track of those. \
Then, for each cube, I check for all its neighbours whether it is in the outside class or not. 
