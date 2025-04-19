# `sandybot` chess engine
hi! this is my chess engine project :)

this engine is written with one sole principle at its core: ***speed***

I try to improve it on all aspects, insofar as the code to do so is as performant as could possibly be (given that I also need to study sometimes).

see performance on [lichess!](https://lichess.org/@/sandybot)


## Project Structure
the engine has grown somewhat haphazardly, and I could only ever describe it as a monolithic architecture. still, there's some effort in splitting it into modules, so here they are:
### Engine
this is why you're here, right?

the `Engine` is a (singleton) struct instance that keeps track of the position and maintains one transposition table until it gets dropped. 

*but wait, isn't UCI a stateless protocol???*

**yes, but!**

old entries in a transposition table can never harm a future search (with the exception of hash collisions). among other things we can keep track of, it gives us enough valid reasons to claim *since we* ***can*** *keep a state, and can use it to help us, we do*. in fact this practice is basically universally assumed when the UCI `ponder` is used.

#### Evaluation
the evaluation function computed at every leaf node resides in [`./src/engine/evaluation/mod.rs`](src/engine/evaluation/mod.rs)

everything else in the module is a helper to the main `evaluate()` function

#### Move Generation
deceptive name since I use [`jordanbray/chess`](https://github.com/jordanbray/chess) for the actual *generation* of moves (as well as for board & bitboard representations). this module is responsible for *move ordering*, ie giving the moves to the search function in order from best to worst, based on a heuristic guess.

since alpha/beta pruning relies on this, it has a **huge** impact on engine performance.

#### Search
arguably the main part of the engine.

- [`./src/engine/search/main_search.rs`](src/engine/search/main_search.rs) contains the root-level search,
- while [`./src/engine/search/negamax.rs`](src/engine/search/negamax.rs) contains the recursive `negamax()` search function
    - the base algorithm is heavily based on the (exceptionally well explained) [wikipedia.org/wiki/Negamax](https://en.wikipedia.org/wiki/Negamax)

#### Setup 
contains the struct and enum primitives I use throughout the engine, mainly `Value`, `Depth`

#### Transposition Tables
- [`src/engine/transposition_table/mod.rs`](src/engine/transposition_table/mod.rs) contains the traits that the engine relies on to use a TT in the search functions.
- [`src/engine/transposition_table/entry.rs`](src/engine/transposition_table/entry.rs) contains the struct `TableEntry`, a very compact (128 bits incl full hash key) representation for entries in the transposition tables.
- [`src/engine/transposition_table/empty_table.rs`](src/engine/transposition_table/empty_table.rs) is a no-op implementation of the transposition table traits defined in `mod.rs`
- [`src/engine/transposition_table/vl.rs`](src/engine/transposition_table/vl.rs)  is a trivial `RwLock` + `Vec<TableEntry>` implementation of the TT traits, which internally is just a bare-minimum hash map.

there is plenty of room for improvement here! 
check out the issues for desired implementations

### Testing / Benchmarking
#### Unit tests
in every module there are unit tests at `src/[module]/tests/[unit].rs`, 
referenced from the bottom of `src/[module]/[unit].rs` using
```rust
#[cfg(test)]
#[path = "tests/[unit].rs"]
mod tests;
```

#### Integration tests
As per rust standard practice, integration tests are in [`./tests/`](tests/).

Currently this includes
- [setup code](tests/shared/mod.rs)
- [test for finding correct mating sequence](tests/mate.rs)

The rest of the files there are part of [[#Benchmarks - github actions]]

#### Bechmarks - `cargo bench`
under [`./src/benches/`](src/benches) (predictably) 
#### Benchmarks - github actions
A harsh realisation is that there is unfortunately no one way to benchmark chess engine internals individually.
Even put together, it is still hard to know how well the engine is performing, without actually playing full games.

For most PRs though the focus is on a single component, and a side-by-side comparison of the changes with the target branch would be really nice ;)

Check out any PR with comments from github-actions bot to see this in practice

### Frontend - CLI / UCI
everything under [`./src/sandy/`](src/sandy/) is part of the frontend (almost)
- [`./src/sandy/player/`](src/sandy/player/) is for playing against the engine CLI using a TUI
- [`./src/sandy/uci/mod.rs`](src/sandy/uci/mod.rs) handles the UCI part of the CLI: converting commands to engine internal instructions
- [`./src/sandy/uci/time_control.rs`](src/sandy/uci/time_control.rs) passes UCI time controls (eg `go btime 1000 wtime 1000`) to the engine, while also heuristically calculating how much time the engine should think for
- [`./src/sandy/uci/search_controls.rs`](src/sandy/uci/search_controls.rs) does the same for search controls (eg `go depth 4`)

## Changelog
- `v0.6.3` TBD, TODO
- `v0.6.2` inline move ordering: switch from an allocated `Vec<ChessMove>` to an iterator that only generates moves as needed, performing all move ordering operations on the construction of the iterator.
- `v0.6.1` variable search depth: when a node has <= 3 children, increase search depth by 1, just for this case. this massively helps lookahead in positions with a lot of checks 
- lost versions: i did not actually keep a changelog until `v0.6.1`. i do not remember the details here
- `v0.1.0` initial implementation, august 2023. it was not tested and prevalent logic errors meant it essentially played randomly
