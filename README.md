# CELLULAR AUTOMATA

===================

This is a Rust based cellular automata library with a python Interface.

# Model Options

There are multiple configuration options in the model.

## Parallelism

We can configure the model to run in multiple parallel modes.

- Full parallel - Each process is run on each cell indepenently before running any cell updates
- Parallel cells - Processes are ran in series. The cells are updated after each process but cells are ran in parallel.
- Full Series - Each process is ran on each cell with updates after each process and each cell. Cell order is randomized. NOT IMPLEMENTED

Full parallel is the fastest with full series being the slowest. Full parallel can only be ran if all the processes are independent.

# Future features

- Auto detect which processes can be ran in series and which can be ran in parallel.
