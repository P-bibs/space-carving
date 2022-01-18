# Space-carving

An implementation of [A Theory of Shape by Space Carving](http://www.cs.columbia.edu/~changyin/paper2read/SeitzICCV1999.pdf) in Rust.

## Preview

Reconstruction target:

<p align="center">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/dino.jpg" width="300px">
</p>

Result:

<p align="center">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/dinofront.png" width="400px">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/dinoback.png" width="400px">
</p>

Carving process:

<p align="center">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/render.gif" width="500px">
</p>

## Dependencies

You'll need `rustc` and `cargo`. [Download instructions can be found here](https://www.rust-lang.org/tools/install).

The output `.ply` file can be viewed in most 3d software, but I've found [Meshlab](https://www.meshlab.net/) to be reliable.

The visualization Python scripts were tested on Python3.6 and need a variety of dependencies that can be found in the `scripts/requirements.txt` file.

### Docker

Alternatively, a dockerfile is provided that includes most of the dependencies. You can build and run it as follows:

Build:

```bash
docker build -t space-carving .
```

Run:

```bash
# Start container
docker run --mount type=bind,source="$(pwd)",target=/home/space-carving --name space-carve -d -t space-carving

# Attach to it in bash
docker exec -it space-carve /bin/bash
```

You can now `cd /home/space-carving` inside the container and run the project. You'll still need to install the python dependencies with `pip` if you want to run the visualization scripts.

## Running

Enter `cargo run -- --help` to see possible command line arguments. An example invocation with full arguments is below:

```bash
cargo run --release -- --dataset dinoRing.json --num-images 40  --output carved.ply
```

Command line argument need to be entered after `--` to separate them from `cargo` arguments. Only the `--dataset` and `--num-images` flags are required.
