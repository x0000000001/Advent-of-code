# README

Large graph exploration as day16.
I characterized a node of the graph by :

* the time remaining
* the collected ores
* the built robots

Optimizations (as commented in my code): 

* **general idea** : I explore from robots built to robots built, I never just decrement the time and increase the ores.
* **optimization 1** : If we can build a geode robot without waiting, useless to explore other paths than building a geode robot (since we can produce only 1 per round) 300s -> 60s
* **optimization 2** : The three first ressources are used only to build other robots it is therefore useless to have more robots of one of this type than the maximum ressources needed to build other robots (since we can produce only 1 robot per round) 60s -> 3s
