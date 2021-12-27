# `rcc`: A C++ project manager in Rust

Tired of having to type out every single file that you need to
include in compilation? Don't feel like learning to use `make` or
`cmake` since you've got other better things to do? Feel like having
a compilation tool that you've coded yourself instead of using tools
that have been developed for many years and are probably more advanced
than your tool ever will be? Well, this may not describe you (in fact, 
it probably doesn't). But it definitely describes me!

So I'm making my own C++ project manager for my college classes.
Nothing super complicated, just a fun little project. (And I get to
code it in Rust!!)

## Roadmap
- [ ] Compile and run one file
- [ ] Compile and run a directory
- [ ] Compile and run based on a config file
- [ ] Incremental compilation
- [ ] Handle errors from running `g++` better
- [ ] Testing should use `Path`s, not `String`s
