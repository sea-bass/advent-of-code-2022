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
