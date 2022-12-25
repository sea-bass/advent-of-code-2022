# Notes
This will capture my learning process as I go through each day starting with zero Rust knowledge.

These notes will certainly help me learn, but they will also help me look back and cringe at how little I knew and (probably) how incorrect my terminology was. I hope you, the reader, can have a laugh at my expense.

---

## Day 1
My first impression was not great due to the nature of the challenges -- specifically, that the first thing needed is to load data from a file. I immediately thought "wow, if file I/O is this hard in Rust I'm in for a bad time".

However, passing that learning curve I realized that the rest was quite nice. I'm sure I'm not using any of the best practices or anything, but by searching around I was able to get a half decent solution in a reasonable time.

My impression at the end of the day? As I expected, Rust is like a designer baby between Python and C++ (or a general-purpose scripting language and a strongly typed system language). 
* Compared to C++, I enjoy that all the defaults are basically flipped. `let` by default infers data types and makes variables immutable, thus recreating the often used `const auto` qualifiers in C++.
* Things that are still relatively modern in C++ are everywhere in Rust. I wasn't expecting to have to deal with [optional values](https://doc.rust-lang.org/std/option/) and [results](https://doc.rust-lang.org/std/result/) in my "hello world", as the C++ analogs (optional and expected) are C++17 and C++23 features, respectively.
* Printing is very nice in Rust. You get, for free, functionality on par with Python f-strings and the C++ {fmt} library. I like.
* Lastly: The compiler warnings and errors are AMAZING.

## Day 2
My biggest fixation today was that I expected importing external modules (or "crates", as they are called) to be as easy as Python where you just `pip install blah` and you can immediately `import blah` in the code. Not the case (at least as far as I could tell).

After a few attempts to grab functionality that my searches yielded could be imported from external crates (namely, converting between char and ASCII and a modulo operation), I gave up and decided to just hack a solution without any other crates.

Sidebar: **WHY IS THERE NO MODULO OPERATOR IN RUST?** I definitely submitted a wrong answer because I was using `%` as modulo, not as remainder like it truly is. Not blaming Rust exclusively for my troubles; I also had other bugs leading to wrong submissions.

After getting an answer, my cognitive load was sufficiently reduced where I wasn't trying to balance solving the puzzle with importing utilities, and landed on this [Organizing Code](https://rust-classes.com/chapter_4_3.html) resource. This will be nice to look into later, both for splitting code into multiple files and for declaring external crates as dependencies. My impression here is that once a project is set up, `cargo build` should do its thing and I will be happy. We'll see...

## Day 3
As promised, I converted my repo into a Cargo workspace so now I can build and run things way more easily!

The first part of today's puzzle was quite straightforward as it reused a lot from the last 2 days: Read a file line-by-line, do some operation on strings to convert them to a numeric score, and call it a day.

The second part I struggled with because it involved reading a file multiple lines at a time, and I wanted to make this generic to any number of lines just for myself. Now that the project was all "Cargofied", I was able to grab [`itertools::chunks()`](https://docs.rs/itertools/0.7.8/itertools/trait.Itertools.html#method.chunks) to do exactly what I wanted... except not really.

* Since I was reading using `BufReader`, the iterable you get from that `lines()` method didn't seem to be compatible with `itertools::chunks()`.
* No big deal -- I just read the entire text file as a string and split it by newlines, which let me use `itertools::chunks()` perfectly... except I found it very difficult to know how to use this `Chunks` object and eventually gave up since I was pushing 2 hours on this.
* ... but now that I had a vector of `&str`s, I could easily use slices to do my bidding and that was okay even though it requires explicit indexing and I'm sure there is a smarter way of doing these things.

On the plus side, I enjoyed a lot about the Cargo system. This is exactly the ease of use that I was told about by my colleagues! Specifically,
* Running `cargo add itertools` modified the `Cargo.toml` and `Cargo.lock` files automatically.
* Running `cargo run --bin my_binary` automatically recompiles that binary if it has changes.

My takeaway for today was that I found myself defaulting to indexing and range based for-loops, and that immediately raises an alarm in my head that I'm not taking advantage of what Rust has to offer. Today's mission to set up Cargo and solve the puzzle was successful, and the next steps are clear.

UPDATE: After some colleagues added suggestions, I was able to eliminate all `len()` functions and range-based for-loops and indexing. I also learned about the `match` operator. Still much more to improve!

## Day 4
Not much to share for today's progress. This puzzle went back to reading a file line-by-line, so I could draw upon existing patterns.

I did learn (read: search Stack Overflow) how to [split a string by multiple delimiters](https://stackoverflow.com/questions/29240157/how-can-i-split-a-string-string-or-str-on-more-than-one-delimiter), but most importantly it was my first use of this [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) functionality I had seen around the place, to transform an iterator into another. In my case, I used it to convert strings to integers to solve today's puzzle.

I also brought back importing an external crate so I could use [`itertools::next_tuple()`](https://docs.rs/itertools/0.10.5/itertools/trait.Itertools.html#method.next_tuple) to get a tuple of named variables and make the code look a little more readable than just indexing into a collected vector.

I'm now going to take a step back and read through Rust content to learn about some key concepts and features I should be aware of... I feel I'm still not being as idiomatic as I could be.

## Day 5
Today I took that step back and began reading [The Rust Programming Language book](https://doc.rust-lang.org/book/title-page.html).
It has been tremendously helpful to understand the core concepts behind Rust, and what I was leaving on the table.
So far in my reading, what has stood out as unique functionality is:

* Ownership ([Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)) and its implications towards creating safe code that catches what would be potential runtime issues at compile time.
* Enums ([Chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html)), which solidify how ubiquitous things like `Result` and `Option` are actually implemented in Rust, what all the syntax means, etc.

Sadly, the reading didn't seem to have taken effect because my puzzle solution was abysmal and it took me 2.5 whole hours to do it all... even with some help getting pointed to the `Vec::split_off()` function.
Overall, not pleased with my puzzle solving skills today.

On the more positive side, I did implement my first structs today which was good practice to how Rust does abstraction.
Excited to do more with enums going forward -- not just being better about handling the existing ones better (right now I've just sort of been spamming `unwrap()` under the assumption that the data is there), but creating my own enums to help do my bidding for the remaining puzzles.

Another observation is that there is a lot of copypasta between parts of the same day. So going forward I may consider reusing the same file and adding arguments to toggle which part is being run.
Should be good practice.

## Day 6
I found today's puzzle much more manageable than yesterday's.
Funny enough, my plan for today to unify both parts into one file was perfect given the puzzle,
as the first and second part were the same question but with a different range of values.
Not much else worth noting for today besides the fact that I tried out `HashSet` as my approach to convert a vector slice to something that's easy to detect duplicates for.

My initial approach involved sort of giving up and creating a blank `HashSet` then iterating through a slice and adding the elements piece by piece.
However, after speaking to a colleague, he implored me to look into `from_iter()`, which was super helpful.
I fumbled my way through getting it working in a nice idiomatic way, but seeing all these error messages about traits leaves me yearning for knowledge of what a trait actually is, which I know comes up later in the book.

Generally, I must read more of the book... there's also a lot more I want to know about collections in Rust even though I've been using them somewhat blindly.

## Day 7
I spent a LONG time trying to crack this, and ran into fundamental issues with picking a solution that would work in other languages, but Rust actively tries to avoid.
Specifically, I attempted to have a data structure and a `HashMap` that helped me find things within that data structure.
However, this multiple ownership business ran me into a wall and I have up... for now.

UPDATE: After getting a hint from a colleague, I gave up on the custom data structure and was able to get a much more concise solution using a stack (represented by a `Vec`) and a `HashMap`. This was a tough one.

## Day 8
This puzzle was again more on the straightforward side.
Not much was new here in my learning except this was my first instance of making a container of containers (in this case a `Vec<Vec<u32>>`).
There was probably an opportunity to not copy-paste so much of my code for all the different cases in the puzzle (left, right, up, and down directions), but I didn't feel the extra time spent would get me to learn anything new.

## Day 9
Continuing on the trend of puzzles being doable but still challenging, after a slightly soul-crushing Day 7 experience.
Today I tried out tuples as a simple way of representing (x, y) position as an `(i32, i32)`, which was a nice new thing but nothign groundbreaking given I've been using more complicated data structures.
One interesting observation is that eventually I wanted to convert my tuple to a `Position` struct with `x` and `y` field so readability would be improved (`variable.0` would become `variable.x`, for instance) ... however, since I was using a `HashSet` in my implementation it seemed that I needed to write my own hash trait for that position class.
Based on [the documentation](https://doc.rust-lang.org/std/hash/trait.Hash.html), it seems pretty straightforward; however, I then ran into issues with having to implement equality traits and more, so I dropped it in favor of staying with the simple tuple type.
Luckily, [type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html) was a simple way to get some readability out of this!

## Day 10
I'm happy to say 10 days in that I only Googled **one** thing in the solving of this entire exercise, which is wildly different from my experience so far.
Maybe today's problem was more manageable (I think it was), but maybe my efforts learning Rust are starting to pay dividends.
I don't think I used any new functionality today, except that the ~~modulo~~remainder operator that gave me grief early on resurfaced and was useful as an actual remainder operator.

## Day 11
This was another extremely tough day for me, just like Day 7, taking me around **5 hours total** even with help from a colleague at the end!
To parse these arbitrary operations, I found a really cool crate named [`evalexpr`](https://docs.rs/evalexpr/latest/evalexpr/) that I eventually figured out how to utilize properly given how Rust enforces ownership.
I also got more familiar with structs and closures in Rust doing this challenge, which has been a good learning experience.
However, moving to Part 2 had me hitting against a fundamental limitation in `evalexpr` where there is no notion of "big integer" support, and simply converting from ints to floats was not enough... so I'd have to think of something else altogether.

UPDATE: The modulo operator on the lowest common multiple of all the monkey's divisors. That was the trick. Could keep everything as `i64` once my colleague gave me that hint!

## Day 12
Of course there was going to be a graph search algorithm question...

I didn't do anything super fancy in terms of algorithms; I simply wanted to implement depth-first search (DFS) using some of the collections provided by Rust.
In this case, I ended up using a `VecDeque` to push to and pop from, and then a `HashSet` on the node positions to figure out whether nodes were already added to the queue.
I got stuck for a little bit since I was accidentally adding nodes multiple times to the queue, which still solved fine for the test input but definitely caused the puzzle input to add nodes faster than it could consume them... so that took a while to debug.

## Day 13
This is starting to get really leetcodey, which I don't particularly enjoy...
Today's puzzle involved writing a recursive parser for arbitrary levels of brackets, plus a sorting algorithm for the second part (ugh).
The parser was a little tricky but I'm glad I stuck it out; as far as the sorting algorithm, I literally just copied a Rust bubble sort implementation from [this page](https://www.hackertouch.com/bubble-sort-in-rust.html) and it did the job.
It did make search for the second part comically slow (took a few seconds to solve), but that's fine by me. Ship it.

## Day 14
Today was a fun puzzle, but still took me a while.
The Rust highlight for today was I was sick of using `Vec<Vec<TYPE>>` for 2D arrays, so I added the [`ndarray`](https://docs.rs/ndarray/latest/ndarray/) crate and my life was made way easier.
I ran into a little snag during this puzzle because for some reason I had originally assumed that the grid was finite width and sand grains could overflow on the left/right sides (which was mentioned nowhere).
This meant that for Part 2 my simulation wouldn't end, and it took me a while to realize why.
As a super hack, I basically added an "offset" to the grid so X=0 was some other large positive value, and just kept padding all my coordinates until the Part 2 simulation for the puzzle input terminated correctly.
Whatever, it worked :)

## Day 15
Oh, more 2D grids in today's puzzle!
Unlike yesterday, today's puzzle didn't have walls or anything like that, so I realized we don't have to store every possible grid element and for my solution a `HashMap` was enough... although I was terrified that Part 2 was going to sneak something like this that would break my assumptions and send me back to the drawing board.
Thankfully, not the case, as validated by [this Reddit post](https://www.reddit.com/r/adventofcode/comments/zmfo0j/2022_day_15_part_1_ah_yes_this_seems_to_be_a_bit/).

However, there was another issue.
There definitely was something more clever on the algorithmic side I could have done, because Part 2 was very, very, very slow with the full puzzle input with the `O(n^2)` loop over all rows, all columns within range.
Even running with the release profile, I did the math and it would take ~100 hours to complete with my implementation.
So that's not going to fit within the day...

**UPDATE:** After scouring Reddit, I realized the logic is to assume that the beacon must be in a point immediately outside any individual sensor's periphery, and then to check only those peripheral points and compare that with the range of all the other sensors.
Using perimeter vs. area = computational complexity gains. Lesson learned.

## Day 16
More hardcore algorithms questions. What a drag.
I tried pruning the search space by doing a bunch of things, such as:

* Not even attempting to open valves with 0 flow rate
* Avoiding cyclic transitions where you just go back and forth between tunnels
* Dealing with the fact that transitions don't matter when you have open
* Having an optimistic flow rate that checks if you were to open all valves at this step, would it be enough to beat the max score so far?

Besides that I didn't really have the energy to figure out anything more clever.
The solutions still took a very, very, very long time.
I read on Reddit that one approach to reduce the search space is to eliminate all the zero flow valves and build a simplified graph of only transitions between "important" valves using the shortest path between them.
I originally said I wasn't going to spend hours on that, but I definitely spent a longer time with the heuristics above and fixing bugs... so who's the real winner now? Not me.

On a positive note, I grabbed the [`regex`](https://docs.rs/regex/latest/regex/) crate so I could be just a little bit smarter about parsing the file input, and that worked great!

## Day 17
This problem seemed really fun, and working on Part 1 was very fun, although time-consuming given my skills.
Yet again, Part 2 throws a practical wrench in things because you can no longer allocate memory to contain everything.
I made a really stupid rolling buffer that literally copies elements instead of just moving the indexing.
It was (as the trend seems to be going) painfully slow, but it will eventually solve the problem in about 100 hours, according to rough calculations.

Looking at Reddit again, I see the "discrete math PhD" solution is to apply a cycle finding algorithm so you can find a repeating pattern well before the cutoff and then use some simple arithmetic to find the tower height after any arbitrary steps given the pattern.

Am I going to go down this rabbit hole, though? Yes.

After taking a break, I spent a few more hours writing up a (probably inefficient) pattern finding algorithm, which sped up the code so much that by the time I figured out the algorithm, fixed bugs, etc. my brute-force approach got approximately 2.8% through all the required steps.

## Day 18
A manageable problem at last!

Part 1 took me around 15 minutes to complete which was a well-needed ego boost after the last few days have had me doubting myself.
Hash sets made for a very efficient and quick to implement solution.

Part 2 was a little trickier and I may have gone down the wrong path with trying to actually create a full 3D array, for some reason.
After burning some time doing that, I realized one could just iteratively expand all unoccupied nodes from our current face and check whether there is any adjacent face that ends up reaching the outskirts of the grid.
Some silly logic bug fixes later, a solution was born!

I did "cheat" a little in visually inspecting the data and seeing that all the grid values were in the range `1..20` for all problems... but if that were not the case, it would be relatively straightforward to compute the min/max XYZ limits as we're parsing the data in linear time.

Lastly, I ran into a minor snag where I was not able to push structs, even if cloned and the `Copy` and `Clone` traits were derived.
Worked around it by converting the struct to a tuple of data for that part of the code, but obviously it would have been nice to not do that.

## Day 19
Part 1 seemed to be a similar "simulate and keep track of max value" problem, like on Day 16, except this one had more interesting dynamics and was overall slower to get an answer.

Since the blueprints can all be executed in parallel to produce results, I decided to look up what Rust had to offer in terms of parallel iterators.
Lo and behold, I was able to grab the [`rayon`](https://docs.rs/rayon/latest/rayon/index.html) crate to parallelize my entire implementation!
This was especially helpful for Part 1 in which we're going over 30 blueprints in parallel.

```rust
fn get_quality_level(blueprints: &Vec<Blueprint>, n_steps: u32) -> u32 {
    blueprints.par_iter()
              .map(|b| b.id * get_max_geodes(&b, n_steps))
              .sum()
}
```

For Part 2, however, pruning search was the key.
I came up with one pruning strategy to keep track of the max number of geodes at each step.
If any node expanded had less geodes than this running max, it's necessarily not on the most efficient and we can blow through all those expanded states.
This helped (once I realized I had a bug that over-pruned for the Part 2 puzzle input), but things still got pretty slow moving to the second part, where the horizon time increased.

After going on Reddit for more pruning strategy ideas, there was a really nice one I overlooked, in which we don't want to expand any more nodes if our material production already exceeds the necessary production for any robot we want.

## Day 20
This was an interesting problem because it immediately indicates you should try to in-place swapping of a list or you're going to have a bad time.
Rust happens to have a [`swap`](https://doc.rust-lang.org/std/mem/fn.swap.html) function that operates on containers such as the `Vec`s I ended up using, so that part was fine.

While it took me a while to get the swapping logic right when the values wrap around (thank goodness for test examples), the rest was relatively smooth besides dealing with wrapping and overflow.

I had an embarrassing computational bottleneck that turned out to not even be algorithmic!
For some reason, I had changed something that used the remainder (`%`) operator to a while-loop to debug something, and thought nothing of it.
After some trial-and-error profiling, I found this and my code sped up a ridiculous amount.
Small victory!

## Day 21
Mmmm, recursion.

When I started reading this puzzle, I immediately got on the defensive about using a `HashMap` to keep track of my data out of fear that I'd be driven into out-of-memory issues for Part 2 and I'd have to reimplement it as an iterative solution.
However, my intuition won out and there was no slow computation to deal with today; only bugs in my implementing the inverse operations for the second part.

I was also apprehensive about using the `evalexpr` crate again since they did not support any integer data types beyond `i64`, and that got me last time before I fixed my solution with the `%` operator.
Fortunately, the parsing didn't need anything fancy here and a simple `split_whitespace()` + `match` block did the trick.

Overall, not a bad day. It was focused on getting the algorithm right, so I felt I was spending my time thinking and not bashing my head into a wall.

## Day 22
The first part of this puzzle seemed fun just like the prior day, where it took a while but I felt motivated to get through it and implement a working solution ... however inelegant.

For parsing the input which alternates between numeric and non-numeric string portions, I couldn't find anything better than [this Stack Overflow question](https://stackoverflow.com/questions/32257273/split-a-string-keeping-the-separators) proposes, so I shamelessly copied it over into my solution.
For representing the world, the `ndarray` crate made a reappearance and it worked great!

The weird cube geometry of Part 2 was honestly a slog... but at least with hard-coded rules for the puzzle input, a paper cube, and a lot of patience, we got there!

## Day 23
This was another fun puzzle throughout!
No real surprises or weird computational/memory bottlenecks crept up in Part 2, which was appreciated as we approach the end of this event.

In this puzzle, I was deliberate about not representing the state as a 2D array using something like `ndarray` because it was evident that the grid could just keep expanding and keeping track of indices would be a pain.
So, a simple data structure that just contains each elf's position did the trick, even if there were lots of linear-time searches.
I considered maybe using a `HashSet` to identify duplicates, but having indices seemed useful in this case, so pure `Vec` it was.
Even so, Part 2 didn't take offensively long -- about 20 seconds with the release profile.

## Day 24
This puzzle was fun and has the kind of thing I like (simulate weird dynamics until an answer comes up).
However, it took me longer than expected simply because I had a few silly bugs that took me a while to realize.

First, I was implementing DFS instead of BFS accidentally by popping off the wrong side of my queue. Embarrassing.
This led me all the way to implementing A* with a priority queue from the [`priority_queue`](https://docs.rs/priority-queue/latest/priority_queue/) crate, but once I realized my folly I backpedaled and did a regular `Vec` with an accompanying `HashSet` to check for duplicates.
This makes it the second (or third?) time this challenge that I brought in a priority queue and then realized it wasn't necessary.

Secondly, in my transition model I wasn't factoring in that staying in place was not always a feasible transition because blizzards also come to the elf's current position.
Also embarrassing.

Once I figured these two things out, the path from Part 1 to Part 2 took just a couple minutes.
Simply had to update the function signature to change the initial/goal positions as well as return the final blizzard state for the start of the next round.

One more to go -- home stretch!

## Day 25
In concept, this last problem was meant to be easy, in that it required converting some weird base + offset numbers to decimal, adding them up, and then converting back to that other encoding.
However, I got stuck in the re-encoding at the end and was taking on a much more complex approach than was necessary.
Should have started by searching the Internet.

Anyways... **Advent of Code 2022 is officially completed!**

I certainly feel much more comfortable with Rust, in that I had never used it at all before this challenge.
There's still much more to know, but this has formed a good basis and I look forward to using it for personal work in the future.
