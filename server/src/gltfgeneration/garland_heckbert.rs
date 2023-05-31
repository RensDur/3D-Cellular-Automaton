use super::quadric_vertex::{QuadricVertex, QuadricTriangle, QuadricVertexPair};




pub struct GarlandHeckbert {

}




impl GarlandHeckbert {

    /**
     * Main method to perform this algorithm.
     * Execution of this algorithm heavily relies on the datastructures
     * specified in quadric_vertex.rs
     */
    pub fn simplify(vertices: &[f32], indices: &[u32], threshold: f32) {

        let threshold_sqr = threshold * threshold;

        //
        // Step 1: Create an array of all vertices, using the QuadricVertex datastructure
        //
        let mut quadric_vertices: Vec<QuadricVertex> = vec![];
        let mut quadric_triangles: Vec<QuadricTriangle> = vec![];

        for v in (0..vertices.len()).step_by(3) {
            // v    vertex.x
            // v+1  vertex.y
            // v+2  vertex.z
            quadric_vertices.push(QuadricVertex::new(vertices[v], vertices[v+1], vertices[v+2]));
        }

        for i in (0..indices.len()).step_by(3) {
            // i    index@v1
            // i+1  index@v2
            // i+2  index@v3
            quadric_triangles.push(QuadricTriangle::new(
                quadric_vertices[i],
                quadric_vertices[i+1],
                quadric_vertices[i+2]
            ));
        }

        //
        // Step 2: Compute Q-matrices for every vertex
        // &
        // Step 3: Compile a list of valid pairs
        //

        let mut valid_pairs: Vec<QuadricVertexPair> = vec![];

        // Loop over all vertices
        for i in 0..quadric_vertices.len() {

            // Compile a list of all triangles share this vertex
            let mut shared_triangles: Vec<&QuadricTriangle> = vec![];

            // Loop over all triangles
            for t in &quadric_triangles {

                // If this triangle contains this vertex
                if t.contains(&quadric_vertices[i]) {
                    // Add this triangle to the list
                    shared_triangles.push(t);
                }
            }

            // Use these triangles to compute and store the qmatrix for this vertex
            quadric_vertices[i].compute_and_store_qmatrix(shared_triangles.as_slice());

            // Also use this list of triangles that share this vertex to determine valid pairs
            // with this vertex because they form an edge
            for t in shared_triangles {
                for pair in t.create_pairs_with_vertex(&quadric_vertices[i]) {
                    if !valid_pairs.contains(&pair) {
                        valid_pairs.push(pair);
                    }
                }
            }
        }


        //
        // Step 3: Compile a list of all valid pairs (continuation)
        // All vertices on edges are already part of the valid_pairs array.
        // Add the remaining ones using the threshold distance
        //

        for v1 in 0..quadric_vertices.len() {
            for v2 in 0..quadric_vertices.len() {
                // If the distance squared between these two vertices is smaller than the threshold squared
                // AND v1 != v2
                // AND they're not an edge (aka already in the list)
                // Add this pair to the list as well

                if v1 != v2
                   && quadric_vertices[v1].distSqr(&quadric_vertices[v2]) < threshold_sqr {

                    let pair = QuadricVertexPair::new(quadric_vertices[v1], quadric_vertices[v2]);

                    if !valid_pairs.contains(&pair) {
                        valid_pairs.push(pair);
                    }
                    
                }
            }
        }


        //
        // Step 4: Sort the valid pairs on lowest cost first
        //

        valid_pairs.sort_by_cached_key(|a| a.cost() as i32);

        





    }

}