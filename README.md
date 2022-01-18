# Space-carving

An implementation of [A Theory of Shape by Space Carving](http://www.cs.columbia.edu/~changyin/paper2read/SeitzICCV1999.pdf) in Rust.

## Dependencies

You'll need `rustc` and `cargo`. [Download instructions can be found here](https://www.rust-lang.org/tools/install).

The output `.ply` file can be viewed in most 3d software, but I've found [Meshlab](https://www.meshlab.net/) to be reliable.

The visualization Python scripts were tested on Python3.6 and need a variety of dependencies that can be found in the `scripts/requirements.txt` file.

## Running

Enter `cargo run -- --help` to see possible command line arguments. An example invocation with full arguments is below:

```bash
cargo run --release -- --dataset dinoRing.json --num-images 40  --output carved.ply
```

Command line argument need to be entered after `--` to separate them from `cargo` arguments. Only the `--dataset` and `--num-images` flags are required.
