# Space-carving

An implementation of [A Theory of Shape by Space Carving](http://www.cs.columbia.edu/~changyin/paper2read/SeitzICCV1999.pdf) in Rust. The provided code contains a module to

- parse images and metadata from the [Middlebury Multi-View Dataset](https://vision.middlebury.edu/mview/)
- perform space carving using Seitz and Kutulakos's method
- write the output to a `ply` file that can be viewed in 3d software such as Meshlab.

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

## Reflection

_Note: I will refer to the method from "Photorealistic Scene Reconstruction by Voxel Coloring" as Voxel Coloring and the method from "A Theory of Shape by Space Carving" as Voxel Carving (both with initial caps to help distinguish them)_

While I have some experience reproducing results from CS papers, this project posed a unique challenge since the relevant papers were published so long ago that I had trouble finding any sort of artifacts besides those contained in the paper directly. This meant that I didn't have a sample implementation to draw ideas from, and instead had to rely on the (somewhat terse) pseudocode given in the paper. This seemed hopeless at first, since the copy of the Seitz and Kutulakos's paper I found was only 8 pages, but I later found an extended 20 page version which gave more details on the consistency check and plane-sweep processes and made my goal more tractable.

### Dataset

The first step was to find a quality dataset. I needed intrinsic and extrinsic data for the images, so I turned to the Middlebury multi-view stereo dataset. While the voxel-carving process should work with relatively few images, the datasets I found had approximately 50 images each which gave me wiggle room. They also contained tight bounding box data which was a useful, though not required, bonus. I then ate up much of my time dealing with the less well documented aspects of the dataset, such as what 3d coordinate system they were using and what image coordinate system the provided projection matrix converted to.

### Voxel Coloring

I then dove into the actual voxel-carving process. While the goal was to implement the methodology from ["A Theory of Shape by Space Carving"](http://www.cs.columbia.edu/~changyin/paper2read/SeitzICCV1999.pdf), I initially began with Seitz's earlier work ["Photorealistic Scene Reconstruction by Voxel Coloring"](https://www.cs.cmu.edu/~seitz/vcolor.html). This method involves going voxel by voxel and reprojecting the voxel onto each image to find the corresponding pixel color. The consistency of the pixel colors are then checked via thresholding the standard deviation, and any inconsistent pixels are carved. Remaining voxels are colored based on the average color of pixels which they backprojected to. While the consistency check is relatively simple, this method has its limitations. Notably, to visit occluder scene elements before the elements they occlude, there must be some view-independent depth ordering on the scene. Practically, this is the case if and only if the scene does not intersect with the convex hull of the camera viewpoints. This was not an issue for me since all cameras were positioned above the scene. Therefore, if I carved from the top plane to the bottom plane, I would always visit occluders before occludees. By masking pixel values once they are correlated with a voxel, we can ensure that occluded voxels can't be matched with a pixel that has already been used to prove consistency for an occluder.

### Exporting

Finally, I wrote a small exporter to `ply` format so I could preview my work in Meshlab. `ply` is a fairly simple ASCII file format, so after some reading of the spec I could consistently output colored voxel meshes. A few optimizations, for example excluding interior voxels, greatly decreased export time and `ply` file sizes.

### Upgrading to Voxel Carving

While I now had the full reconstruction pipeline implemented, the resulting models weren't clean enough for me to feel satisfied. Even after tuning parameters like voxel size, number of input images, and standard deviation threshold for consistency-checking, I could not improve the results to a satisfactory degree. I made the decision to begin upgrading my Voxel Coloring implementation to a Voxel Carving implementation.

Interestingly, Voxel Carving is, for the most part, a strict superset of Voxel Coloring. The Voxel Carving paper goes to great length to give a more general consistency-checking methodology. The result is a consistency-checking function that can take into account complex radiance functions to support scenes with shadows and reflections. However, in the limiting case where we don't have lighting information, we must simply use a Lambertian radiance model where all scene elements reflect the same color in every direction. This is equivalent to the model used in Voxel Coloring. As a result, Voxel Carving did not present any upgrades to my consistency checking function.

However, Voxel Carving did offer an upgrade in terms of the order of the overall carving process. We can see a comparison of the methodologies below:

| Voxel Coloring                                                                                                | Voxel Carving                                                                                                                                                     |
| ------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Single pass plane sweep that considers planes of the scene in increasing distance from the camera convex hull | Perform plane sweeps in each of 6 principle directions (forward and back on 3 axes). Repeat indefinitely until none of the 6 sweeps result in voxels being carved |

The natural question to ask is how Voxel Carving gets around the occluder/occludee issue if planes are not swept in depth order. It turns out, you can preserve the "carve occluders before occludees" constraint by only considering cameras on one side of the plane being swept. This means that Voxel Carving has no constraints on the locations of camera views. While this means that each individual consistency check may have fewer datapoints to draw from, the overall result is a tighter bound on the photo hull since views are considered multiple times.

### Possible optimizations:

- Projecting corners of voxel:

  - When backprojecting voxels, I backproject the center point of the voxel's cube. This means every voxel backprojects to exactly one pixel. A more accurate implementation would backproject each corner of the voxel's cube individually and consider which image pixels the resultant polygon covers. This complicates the implementatoin, since each voxel can backproject to multiple pixels, but improves the output.

- Improved consistency checking (e.g. via [Histogram consistency check](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.67.1990&rep=rep1&type=pdf))
  - The current consistency checking algorithm assumes that a large standard deviation in pixel color values indicates that a voxel is not a part of the scene. However, there are certain cases where valid scene voxels occur in a high contrast area, which means the pixels it backprojects to may have wide ranging colors (for example, many pixels grouped around white and many grouped around black). More advanced consistency checking algorithms (like the one linked above) can recognize that these high contrast areas and rule them as consistent, resulting in a more accurate reconstruction.

## Further Evaluation

Sample meshes can be found in the `meshes` directory. Further previews of reconstructed meshes are below:

Reconstruction target:

<p align="center">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/templeOriginal.jpg" width="300px">
</p>

Result:

<p align="center">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/templeRecon1.png" width="400px">
<img src="https://raw.githubusercontent.com/P-bibs/space-carving/main/media/templeRecon2.png" width="400px">
</p>
