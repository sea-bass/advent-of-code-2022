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
As promised, I converted my repo into a Cargo project so now I can build and run things way more easily!

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