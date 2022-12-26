# Advent of Code 2022
Flavor: **Rust** with **no external dependencies**  
_The use of 'we' on this page is an academic/authorial 'we' — All puzzles were solved individually_  
Solutions are implemented without multithreading and any notes on performance assume the solution is running on a low-to-mid end CPU and running in **DEBUG MODE** i.e. without most compiler optimizations enabled.

# Notes
Thoughts and notes on solutions for each puzzle, e.g. how it was implemented, what could have been done differently, how well does it perform etc.  
Solutions are implemented s.t. the puzzle input can be copy-pasted to stdin and get the result is printed to stdout.
This naturally creates a little extra complexity, since we don't know the size of the input until we've read all of it, and we cannot pre-process the input to formats that may be easier to work with.
### Day 1 - 1
We could have just chained a lot of higher-order functions here, but it was easier to do imperative since we go one line at a time from stdin.
### Day 1 - 2
The solution for this only requires a fixed-size array for the 3 highest elements.  
We could have summed each group of numbers, added them to a vec, then sort the vec and taken the 3 largest element.
### Day 2 - 1 & 2
We just read the input and use pattern matching to tell us what score the different scenarios give.  
We could technically just match on the lines directly instead of splitting them in two.
### Day 3 - 1 & 2
Nothing special here, we just do some ASCII math to convert the characters to their priority.
### Day 4 - 1 & 2
Straightforward, we just parse the ranges and compare the start/end values to find out if they overlap
### Day 5 - 1
While the problem itself is simple, push/pop on some stacks, we need to do a considerable amount of parsing to get the initial state right.
The implementation should work with arbitrary stack count and size.
### Day 5 - 2
Very simple since we can just re-use all the parsing from day5-1, we only need a minor change to make it push/pop multiple elements at a time.
### Day 6 - 1
We treat the input as a Vec of characters and use the `windows` method on the input to get an iterator over all slices that may contain our detection.
After that, we just need to stop at the first slice where all characters are unique.
### Day 6 - 2
All we have to change from day6-1 is the constant, so we simply replace 4 with 14 and we're done.
### Day 7 - 1
Working with trees in Rust can be a bother, so this was solved with a flat data structure.  
We make the (reasonable, by looking at the input) assumption that we do not `ls` the same dir twice.  
For each file, we note where it is and how big it is. For each directory, we note where it is.
We then just run over each dir and check the size of files contained in it. Rust's `PathBuf` makes working with paths really easy for this puzzle.
### Day 7 - 2
We can re-use all the parsing from day7-1. All that's left to do is get the size of the root folder, figure out how much space we need to free, then loop over all directories to find the smallest one that is at least of that size.
### Day 8 - 1
We start off by loading the entire thing into a 2D Vec. 
We can determine how many trees are visible on the edge by just looking at the dimensions and doing some math.
For the inner trees, we loop over all of them and scan the row/column they're on. 
If any tree is the same height or taller, we consider it "hidden" or "invisible".
All that's left is counting how many trees are _not_ hidden and add that to our total  
The implementation wastes some computation and does use quite a bit of brute-force, but it runs virtually instantly regardless.
### Day 8 - 2
We re-use the parsing from day8-1 and change the `check_height` function `check_score`, this time keeping the highest value instead of summing.  
Notably for this part, we have to check FROM the tree TO the edge, since we need to stop on the closest tree that is the same height or taller.
Once that's done, we just sum how far we can see in each direction, multiply them together, repeat for each tree (including the edges) and keep the highest value.
### Day 9 - 1
This one is pretty straightforward.
We keep track of the position of the head and the tail. We move the head 1 unit at a time, then check if we need to move the tail to follow.
If we move the tail, we note down the position it is in.
Once we're done moving around, we de-duplicate the list of positions the tail has been in and count the unique positions.
### Day 9 - 2
This one may seem significantly more complicated than day9-1, but is actually really simple. Instead of keeping track of 1 head and 1 tail, we keep track of 10 'segments'.  
We apply the input moves to only the first segment, then run through the rest of the segments, pretending for each of them that the previous segment is the head.
That means we're essentially just doing the same as in day9-1 but multiple times per move.
After that it is trivial, we just note down the position of the last segment after every move, de-duplicate and count locations.
### Day 10 - 1
We just need to do some math to solve this first part. 
The only slightly tricky part is making sure we run things in the correct order, i.e. we need to make sure we update the signal, sum and counter in the right order.
### Day 10 - 2
At each cycle, before we do any logic, we check if we should draw the current pixel.
All that's left is to run through all the steps, fill pixels and read out the letters.  
Parsing the output from "ASCII-art" to numbers is a bother and must be done manually, but it is printed in a human-readable fashion.
### Day 11 - 1
The trickiest part of this is how to parse the input and how to work with the parsed input.  
We observe the "test" is always modulo something, so we can just store this as a number.
For the "operation", it is always addition or multiplication with either a constant or the input itself.
We check which case we have and construct a function, so we can use dynamic dispatch to call it later.  
We now just parse each monkey into a list, implement the round loop, worry calculation and item passing, which is rather straightforward.
### Day 11 - 2
The only difference from day11-1 is that we no longer divide worry by 3 every time an item in inspected. 
This has the side effect that the worry number grows exponentially larger than before, easily exceeding the limit of a 64-bit integer.  
While this can be solved by just using a "Bigger Integer™" (We would need a _really_ big one), a better solution is to look at what the worry level is used for: we only ever perform modulo on it.   
Since we only perform modulo, we can find the `Lowest Common Multiple` of all the numbers used for the "test".
Whenever we change the worry level, we also perform modulo with the `lcm`, which prevents the number from getting too big, while also not affecting the "test" modulo check.
### Day 12 - 1
We parse the heightmap into a 2D-Vec and use an implementation of Dijkstra's algorithm to find the optimal path.  
For the priority queue, we use Rust's `std::collections::BinaryHeap`.
We take care to only consider neighbouring nodes if they are within an elevation level that we can reach.
Other than that, there is not much more to it than the Dijkstra implementation.
### Day 12 - 2
By default, Dijkstra's algorithm gives us the 'distance' from one node to every other node. 
Instead of going from the 'S' to the 'E' node, we instead find the distance from 'E' to every other node.
Afterwards, we can find each node that corresponds to an 'a' in the input in the distance matrix and take the lowest one.  
We must take care though, since we're now going backwards, that the check for whether we can reach a node is still working correctly.
### Day 13 - 1
The logic for this is relatively simple, but we need to do quite a bit of parsing to get the lists into something we can work with.
Once we're done with the slightly scuffed recursive parser, we can write a similarly scuffed recursive function to check if each item in the list is "correct", "wrong" or if we need to "continue" checking.
### Day 13 - 2
We already have the parsing and comparison code from day13-1. 
We implement Rust's `Ord` and `PartialOrd` to use the comparison function we wrote earlier. 
After that, we can just call `sort` on a Vec with all the lists and then find the indices of the special packets.
### Day 14 - 1
The first thing we do is read in all the lines of rocks.
We then inspect those coordinates to determine the width and height of the cave and allocate an appropriately sized 2D-Vec.  
Next we read run over the coordinates that should have rocks and fill them in.  
Finally, we implement the loop that spawns a piece of sand and keeps trying to move it until it "rests" or falls into the "void".
Once we hit the void, we return the amount of "resting" pieces.  

We actually make the cave 1 unit larger in the left, right and downwards direction than it actually is. 
If a piece of sand manages to fall into this "gutter space", we consider it as being in the void and stop. This saves us from having to do some bounds checks.
### Day 14 - 2
We re-use the same parsing logic, but make some minor adjustments.  
There is an "infinite" floor, but we cannot just make an infinite 2D-Vec. Instead, we observe that a pile of sand of height `n` will have a base with a width of at most `(2*n)-1`.
So, we simply extend the 2D-vec by that size and offset the coordinates s.t. "old" cave is centered in the new, extended one.  
Pieces can no longer fall into the void because of the floor, so a piece will eventually come to a rest at the spawn point.
Whenever a piece comes to rest, we check if it is resting at the spawn point and if it is, we stop the simulation.
### Day 15 - 1
The problem is pretty simple for this first part. 
We just brute-force check all points in the row and count the ones that are covered by an existing sensor/beacon pair.  
This does run relatively slow and wastes a significant amount of work, but it runs fast enough and is so simple that optimizing it is kind of whatever.
### Day 15 - 2
Part 2 requires some more though to solve, since there is now just 1 coordinate which isn't covered, and we need to find it.  
The problem however, is the coordinate space is huge now, 4000000 by 4000000. 
Checking every point by brute force would take a while, even if parallelized on a fast machine.

Since there is only 1 valid coordinate in the whole 4000000 by 4000000 space, the valid location **must** be along the diagonals of at least one of the sensor's area.
The sensors essentially cover a square area, but rotated by 45 degrees.
Everything inside the square is naturally an invalid location, so the only possible valid locations are the coordinates along the edges of the square (that aren't included in the square).

Due to the placement and shape of the sensor areas, the valid coordinate is likely to be at the intersection of multiple sensor's diagonal outlines.
Except for one very specific edge case, it actually has to be at an intersection. We'll get back to that exception.  
We find all the diagonals, compute all the places they intersect, count how many times each intersection appears and check each of them to see if they are the valid coordinate, starting with the "most intersected" one.

It is theoretically possible for the beacon to be in a corner, e.g. (0,0) or (4000000, 0), in which case it would only be on the diagonal of exactly one sensor.
This would likely prevent if from being in any intersections.  
The implementation does not account for this, but the fix is trivial: if we didn't find it by looking at intersection points, look at each of the 4 corners.
### Day 16 - 1
This one took a little longer that I would have liked. 
The solution ended up using bottom-up dynamic programming.

We start at the last round and for each valve (with rate>0, since rate>0 is pointless to open), we check to see how much pressure we can release with each combination of open/closed valves, if we were to open the current valve or move elsewhere.
We repeat this process for each round and in the end we will have a 3D-Vec indexed on `[round][valve][open/closed_valves]` where the stored value is the total pressure.
We look at the round for the valve we start with (AA) and find the highest number among the open/closed combinations.

This naturally uses quite a bit of memory: `(rounds * valves_with_positive_rate * bitset_max_value * sizeof 'total pressure' int)`.
In fact, it will probably use too much memory if we didn't consider only the vales with positive flow, but luckily(?) many of the valves have no flow.

There are definitely better ways to solve this, but considering this (for my input) uses less than 500MiB of memory and runs (in debug) within a few seconds, it's "good enough".
### Day 16 - 2
It took a bit to realize the "trick" to solving this efficiently, but once that's done it's really simple to implement.  

We can re-use all the code from `day16-1`, changing the rounds to 26 instead of 30, since the elephant training takes away 4.
After that, we consider the following: it is not possible for both us and the elephant to open the same valve (or at least, doing so is pointless).
What this means is, we can get away with the disjoint sets of open/closed valves.

In round 26, we loop over all the possible combinations of valves opened by us and the elephant, with no overlapping valves, sum the pressure that we and the elephant release and pick the best pair of sets.
The performance is more or less the same as `day16-1`, since the set check runs very fast compared to the dynamic programming part.
### Day 17 - 1
The first part is relatively simple. We can just simulate the movement of the rocks, one step at a time. 
After every rock settles in place, we make note of how the highest point in the tower so far.
Repeat that for every rock, and we'll end up with the final height of the tower.

Since the tower is not very tall (there is only 2022 rocks), it uses barely any memory to store the full tower and simulating it is fast.
### Day 17 - 2
For this part, we have to figure out how tall the tower will be after 1 trillion rocks, but otherwise the problem is the exact same.  
Now, with 1 trillion rocks it will definitely not fit it memory, and it'll take an excruciatingly long time to simulate.
Instead, we need to come up with a better solution.

That better solution is to detect a "cycle" in the tower. That is, we want to find a part of the tower that repeats itself forever.
If we can find a cycle, all we need is how many rocks were used in that cycle, how tall the cycle is and the height of the tower before we found the cycle.
Once we have that, we can just do some simple math: `height + cycle_count * cycle_height` to get the total height, without needing 1 trillion iterations.

To detect cycles, every time a rock settles, we record the rock type where in the (looping!) input we were and what the furthest a rock has fallen is, as well as the amount of rocks used so far and the height of the tower at that time.
If we see the same rock type, move index and maximum fall, we might have found a cycle. 
We check if it is a cycle, and if it is, do the math described above to get the result.

This runs fast and uses very little memory, as long as the cycle exists early enough. 
This is naturally only the case if the cycle is reasonably early in the cycle, which it "conveniently" happens to be.
### Day 18 - 1
First part is very simple, we construct the 3D grid, loop over each cube and check if there is a neighbouring cube covering the surface in each direction.
We can then just sum the amount of unobstructed sides for each cube, and we're done.
### Day 18 - 2
Part 2 is also relatively simple. As before, we start out by loading the grid.

We maintain a queue of **reachable** cells, which is initially contains all the cubes along the side of the bounding box, since these will always be unobstructed on at least one side.
Additionally, we maintain a map of visited/enqueued cells, so we don't visit the same one twice.  
The 3D grid now contains a *bitset* of which sides are unobstructed. Initially, all sides are considered obstructed.

We keep popping cells from the queue until it runs out.
If the cell is "solid", i.e. contains a cube instead of air, we skip it.
If the cell is air, we update the 6 neighbouring cells. 
If a neighbor is a cube, we update it s.t. it knows that side is unobstructed.
If the neighbor is air, we add it to the queue.

This will eventually process all reachable air cells, giving us a 3D grid of which sides are unobstructed.
We can loop over each cube in the grid and count the unobstructed sides to get our answer.
### Day 19 - 1
Implemented as a recursive search.

At each recursion step, we consider the 5 options we (might) have available: build 1 of 4 bots or wait for time to expire.
For each type of bot, we check if we can build it (are we producing the materials required) and figure out how long it takes to build it (i.e. how many turns we need to have enough materials).
We then recurse, passing along the amount of resources and production values, repeating until it runs out of turns.

We prune a decent number of branches by checking if we'd be producing more of a resource per turn than we could ever use.
It's likely possible to prune more scenarios, but this finds the solution in sub 1-second, which is fine.
### Day 19 - 2
More of the same, really. Instead of doing 26 minutes we now do 32, which significantly increases the (exponentially growing) number of branches.
On the other hand, we only have to check the first 3 blueprints instead of 30, which brings the overall computation required down.

The search implementation is actually identical to `day19-1`. The only difference is we set time to 32 and only check the first 3 blueprints.
Due to the exponentially-growing cost of adding more turns, this is significantly slower than `day19-1`.
However, it finishes within about 30 seconds without any changes, which is "Good Enough™"
### Day 20 - 1 & 2
Pretty straightforward. We load the numbers in, storing where each number was in the original output, and it's actual value.

For part 1, we loop over the list once. We take the numbers as they appeared in the input, find them in the current list and move them by that many places.
After one loop, we simply find the new position of the '0' and sum together the numbers at 1000, 2000 and 3000 places later, wrapping around as necessary. 

Part 2 was trivial since part 1 already kept track of numbers after moving them, relative to how they appeared in the input.
All we need to do is multiply each number in the input and run the "mixing" loop 10 times instead of 1.
Since we already keep track of their original position, we don't need any modifications to run it 10 times.

Part 1 runs in ~0.5s and since part 2 is the same but 10 times, it runs in ~5s. 
There's a handful of simple optimizations we _could_ do, but once again, this is "Fast Enough™"
### Day 21 - 1
For part 1, we parse the monkeys and their "action" into a HashMap.
We recursively resolve the expressions starting with the root.
For numbers, we do not need to recurse, for math operations, we recurse left and right until we get a number, then apply the operation and return the result.  
This isn't gonna recurse more than there are monkeys doing math operations, so it runs virtually instantly.
### Day 21 - 2
We re-use the parsing and part of the solving logic from `day21-1`.

We modify it so `humn` now returns `None`, indicating we cannot solve equations that contain it.
Then we try to resolve the left and right side of `root`. 
Only one of them is going to be solvable, the other one will contain `humn` somewhere.  
We now have the number one of the sides resolve to and know which one contains the `humn` part.  
Starting at the root, we pick the side that contains human and give it the number found for the other side.
We recurse through, applying the _INVERSE_ of the math operations.
For example, if we have `aaaa * bbbb` and `bbbb = 2`, we _divide_ by 2 instead of multiplying when recursing on `aaaa`.
When we reach human, all the inverse operations have been applied, and we have our puzzle result.

When applying the inverse operations, we need to be careful. 
Depending on which side resolves to a number and which one doesn't, we need to perform different operations.

This is not an optimal implementation, but runs virtually instantly regardless.
At the very least, we waste time using a HashMap instead of direct indexing, and we do more recursion than strictly needed.
### Day 22 - 1
Very easy computationally. 
We just move around the map, wrapping around if we need to and if we can.
There's some implementation-wise trickiness in wrapping around correctly, but nothing too bad.

Runs virtually instantly and should work with all inputs...
### Day 22 - 2
Significantly harder than the first part.
We need to fold the map into a cube, then wrap around the sides of the cube.

To make a general solution for this, we'd need to fold an arbitrary cube first, keeping track of which sides connect where in which direction.
This is non-trivial and I only have my own input anyway, so it is instead solved with a significant amount of hardcoding.  
Specifically, it is optimized to solve for only 1 cube net layout:
```
 01
 2
34
5
```
We manually map out which sides connect to where in which direction, then implement a function that performs that mapping.
Some parts can be optimized away, since we only need to consider what happens if we move from a side into "the void".
For example, we only need to consider 'left' and 'right' for `2`, since the 'up' and 'down' sides are already known.

A bit handheld. Runs practically instantly. However, it only works for inputs with the above layout. For posterity, my puzzle input is included as `day22-2/input.txt`.
### Day 23 - 1
We maintain a HashSet keyed on (y,x) coordinate that contain elves.
During each round, we make a HashMap, also keyed on (y,x), that contains a bitset. 

In the first part of each round, we consider where each elf wants to move. 
In the HashMap, we store a bitset. In the location the elf wants to move, we use the bitmap to encode the direction it comes from.
Then during the movement-part of the round, we check each value in the HashMap. 
If the bitset indicates that there are elves from multiple directions trying to move to one position, none of them move.
If there is only one direction stored, we remove the elf on that position from the HashSet and insert a new one, effectively moving it.

We then just run all the rounds, find the bounding box by looking at min/max of the coordinates, calculate the size of that area and subtract the number of elves.  
This runs practically instantly.
### Day 23 - 2
The same as `day23-1`, but instead of stopping after 10 rounds, we stop when no more elves can move, and the answer is the number of rounds it takes to reach that.

The code for `day23-1` actually already includes a check that stops running rounds if nothing could move, so all we need to do is remove the "stop at round 10" code and output the number of rounds instead of empty tiles.

The code is almost the same but runs a lot more rounds. For my input, it takes about 10 seconds.
### Day 24 - 1
We store the blizzards as in a Vec, containing where each blizzard is and which direction it moves. The map is a 2D-Vec.

The map initially has all tiles except the walls as empty. We mark the entrance with a `+` to indicate it is reachable in the first turn.  
For every minute, we create a "new" map, identical to the initial one, and move the blizzards. 
We then place walls on the new map where the blizzards currently are.
Then, on every empty `.` tile, we check if it is reachable from the previous map, i.e. there is a neighbour with a `+`.  
We stop the loop when the destination tile changes to a `+`.

In every iteration of the loop, some tiles are overwritten by blizzards and thus get their `+` removed. 
At the same time, tiles that are reachable this minute change to a `+`. 
This is effectively checking every path breadth-first. Runs in about 0.5 seconds.
### Day 24 - 2
Solved in possibly the laziest way possible.

We duplicate the loop from `day24-1` two times, so we now have 3 loops: `start->end`, `end->start` and `start->end` again.
For the `start->end`, we just flip the check for when we're done and where we start.  
We wipe the map (reachable tiles) but keep the blizzard positions between these 3 loops.

The answer is just the sum of turns taken by the 3 loops. Since this is more or less just `day24-1` running 3 times, it runs in about 2 seconds.
### Day 25 - 1
We parse the SNAFU format to decimal, sum the numbers and use some slightly scuffed code to convert the decimal back to SNAFU.
### Day 25 - 2
We need to _VERY CAREFULLY_ leave our code editor and press the `[Start The Blender]` button.