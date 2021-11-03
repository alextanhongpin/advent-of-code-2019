## Day 18: Many-Worlds Interpretation


Probably one of the toughest problem I faced, because I was bad at shortest path algorithm.

The initial idea is to just to a BFS, and collect the keys along the path, with some minor differences:
- Starting from the entrance, try moving either up, down, left and right
- Stop if you hit a wall, or a door (if you don't have the key)
- Otherwise keep moving
- If you found a key, perform another BFS from the position of the key, and passing through the door now unlocks it


However, it is simply too slow, because we are moving from pixel to another in a single step.

The second attempt now is to first collect all the keys and doors relative to the entrance. This initial flood will also performs a walk to the dungeon in all directions, collecting the `paths` of the dungeon. This way, we can do a check if the path is accessible, by peeking into the head of the vector.

Then, for each key, find the steps from entrance to the key, and then from key to keys. The distance from each key is always the same, so distance from `a` to `b` is also the same as distance from `b` to `a`.
