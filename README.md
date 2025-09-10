# opticore
[hopefully] rust algorithms optimisation library 

### The struct of the repo si the following

```text
optimization-rs/
├── core/          # defines Objective, Model, Solver trait
├── algorithms/    # depends on core, implements Solver for each algorithm
└── examples/      # shows how to use both together