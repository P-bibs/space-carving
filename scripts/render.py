import os
import numpy as np
import open3d as o3d

INTERACTIVE = False

def main():
    

    vis = o3d.visualization.Visualizer()
    vis.create_window()

    vis.get_render_option().load_from_json("scripts/renderOptions.json")

    if INTERACTIVE:
        cloud = o3d.io.read_triangle_mesh("carved.ply")
        cloud.compute_vertex_normals()
        vis.add_geometry(cloud)
        vis.run()
        vis.update_geometry(cloud)
        vis.poll_events()
        vis.update_renderer()
        vis.capture_screen_image("capture.png", False)
    else:
        names = os.listdir("meshes")
        names.sort()

        for (i, name) in enumerate(names):
            cloud = o3d.io.read_triangle_mesh(f"meshes/{name}")
            cloud.compute_vertex_normals()
            print("Read mesh ", i)

            vis.add_geometry(cloud)
            vis.update_geometry(cloud)
            vis.poll_events()
            vis.update_renderer()

            vis.capture_screen_image(f"renders/render%03d.png" % i, False)

            vis.remove_geometry(cloud)

    vis.destroy_window()


if __name__ == "__main__":
    main()
