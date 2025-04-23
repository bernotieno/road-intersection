# ROAD INTERSECTION

This simulation demonstrates a custom traffic control strategy to manage flow at a busy four-way intersection using only red and green lights. The project visualizes real-time traffic behavior using SDL2.

## FEATURES

* Two perpendicular roads with one lane in each direction.

* Traffic lights at each entrance to the intersection (red & green only).

* Vehicles that randomly select a fixed route: left turn, right turn, or straight.

* Vehicles colored based on their chosen route.

* Realistic vehicle behavior: obeying traffic lights, maintaining distance, and fixed velocity.

* Keyboard-controlled vehicle spawning (with safety distance enforcement).

* Clean and responsive graphical interface using SDL2.

## USAGE
1. Clone the repo:
```bash 
$ https://learn.zone01kisumu.ke/git/rotieno/road_intersection.git
```
2. Navigate to the directory:
```bash
$ cd road_intersection
```
3. Execute
```bash
$ cargo run
```
4. Controls
You will use your keyboard to spawn vehicles for your simulation. You will use the arrow keys to spawn a vehicle on the appropriate side of the road, and with a random route.

* ↑ Up: moves towards the intersection from the south.
* ↓ Down: moves towards the intersection from the north.
* → Right: moves towards the intersection from the west.
* ← Left: moves towards the intersection from the east.
* r: moves towards the intersection from a random direction.
* Esc Escape: ends the simulation.

