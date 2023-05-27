use gltf_json as json;

use std::mem;

use base64::{Engine as _, engine::general_purpose as b64};
use json::validation::Checked::Valid;
use miette::{miette, IntoDiagnostic, Result};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
	position: [f32; 3]
}

#[derive(Copy, Clone)]
struct BoundingCoords {
	min: [f32; 3],
	max: [f32; 3],
}

impl BoundingCoords {
	/// Calculate bounding coordinates of a model, used for determining the clipping planes
	fn from_points(points: &[Vertex]) -> Result<BoundingCoords> {
		if points.is_empty() {
			return Err(miette!("At least one point needs to be given to calculate bounding coordinates"));
		}

		let mut bounding_coords = BoundingCoords {
			min: points[0].position,
			max: points[0].position,
		};

		// Loop through the coordinates of all points, and update the minimum and maximum x, y, and z values
		for point in points {
			let p = point.position;
			bounding_coords.min[0] = f32::min(bounding_coords.min[0], p[0]);
			bounding_coords.min[1] = f32::min(bounding_coords.min[1], p[1]);
			bounding_coords.min[2] = f32::min(bounding_coords.min[2], p[2]);
			bounding_coords.max[0] = f32::max(bounding_coords.max[0], p[0]);
			bounding_coords.max[1] = f32::max(bounding_coords.max[1], p[1]);
			bounding_coords.max[2] = f32::max(bounding_coords.max[2], p[2]);
		}
		Ok(bounding_coords)
	}
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
	let byte_length = vec.len() * mem::size_of::<T>();
	let byte_capacity = vec.capacity() * mem::size_of::<T>();
	let alloc = vec.into_boxed_slice();
	let ptr = Box::<[T]>::into_raw(alloc).cast::<u8>();
	let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
	while new_vec.len() % 4 != 0 {
		new_vec.push(0); // pad to multiple of four bytes
	}
	new_vec
}

pub fn generage_large_gltf(input_vertices: &[[f32;3]]) -> Result<String> {

    let vertices_per_buffer: usize = 60000;

    //
    // CREATE N VECTORS OF VERTICES
    //

    let mut vertices: Vec<Vec<Vertex>> = vec![];

    let mut working_vector = 0;

    for v in 0..input_vertices.len() {

        if v % vertices_per_buffer == 0 {
            // We've reached either v=0 or v is a multiple of 60.000
            vertices.push(vec![]);
            working_vector = vertices.len() - 1;
        }

        // Add the vertex at this index to the current working_vector
        vertices[working_vector].push(Vertex {position: input_vertices[v]});

    }

    //
    // CREATE N BOUNDING COORDINATES, THAT MATCH THE VECTORS OF VERTICES ABOVE
    //

    let mut bounding_coords: Vec<BoundingCoords> = vec![];

    for v in &vertices {
        bounding_coords.push(BoundingCoords::from_points(v)?);
    }

    //
    // CREATE A PADDED BYTE VECTOR FOR EACH VECTOR OF VERTICES ABOVE
    //

    let mut bin_base64_contents: Vec<String> = vec![];

    for v in vertices.clone() {
        let bin_content = to_padded_byte_vector(v);
        let mut bin_content_b64 = String::from("data:application/octet-stream;base64,");
        bin_content_b64.push_str(&b64::STANDARD.encode(bin_content));

        // Push these base64 bin-contents to the vector
        bin_base64_contents.push(bin_content_b64);
    }

    //
    // CREATING N BUFFERS OF SIZE 60.000
    //

    let mut buffers: Vec<json::Buffer> = vec![];

    for i in 0..vertices.len() {
        buffers.push(json::Buffer {
            byte_length: (vertices[i].len() * mem::size_of::<Vertex>()) as u32,
            extensions: Default::default(),
            extras: Default::default(),
            uri: Some(String::from(&bin_base64_contents[i]))
        });
    }


    //
    // CREATING A BUFFER-VIEW FOR EACH OF THESE BUFFERS
    //

    let mut buffer_views: Vec<json::buffer::View> = vec![];


    //
    // CREATE AN ACCESSOR FOR EACH BUFFER THAT CONTAINS 60.000 POINTS
    //

    let accessors: Vec<json::Accessor> = vec![];


    //
    // CREATE A MATERIAL
    //

    let mut material = json::Material::default();
    material.pbr_metallic_roughness.metallic_factor.0 = 0.3;


    //
    // CREATE PRIMITIVES FOR EACH ACCESSOR
    //

    let primitives: Vec<json::mesh::Primitive> = vec![];


    // Create a mesh from all the generated primitives
    let mesh = json::Mesh {
		extensions: Default::default(),
		extras: Default::default(),
		primitives,
		weights: None,
	};

    // Create the node that references to this mesh
	let node = json::Node {
		camera: None,
		children: None,
		extensions: Default::default(),
		extras: Default::default(),
		matrix: None,
		mesh: Some(json::Index::new(0)),
		rotation: None,
		scale: None,
		translation: None,
		skin: None,
		weights: None,
	};

    // Create the root that stores all information
	let root = json::Root {
		accessors,
		buffers,
		buffer_views,
		meshes: vec![mesh],
		nodes: vec![node],
		scenes: vec![json::Scene {
			extensions: Default::default(),
			extras: Default::default(),
			nodes: vec![json::Index::new(0)],
		}],
		materials: vec![material],
		..Default::default()
	};

	json::serialize::to_string(&root).into_diagnostic()

}