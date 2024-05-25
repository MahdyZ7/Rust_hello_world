## Basics rust commands

[The Book](https://www.rust-lang.org/learn)
#### 1) using cargo build system 

- To start a new project
```
cargo new [project name]
```

- Build a project
```
cargo build
```
this creates an exutable ```./target/debug/[project name]```

- To run a project
```
cargo run
```

```
cargo test
cargo clippy // checks for commen mistakes
cargo fmt // auto format the code
```
- cargo use for simpelicity so that a work flow looks like
```
git clone example.org/someproject
cd someproject
cargo build
```

#### 2) the c way

a rust file ends in ```.rs```
To compile a rust program use ```rustc [name of program]```
run using ```./[name of program]