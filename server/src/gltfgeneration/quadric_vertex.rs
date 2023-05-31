use cgmath::{Vector4, Vector3, Matrix4, Zero, InnerSpace};



#[derive(Clone, Copy)]
pub struct QuadricVertex<'a> {
    pub pos: Vector4<f32>,
    pub qmatrix: Matrix4<f32>,
    pub link: Option<&'a QuadricVertex<'a>>
}

impl<'a> QuadricVertex<'a> {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: Vector4 { x, y, z, w: 1.0 },
            qmatrix: Matrix4::zero(),
            link: None
        }
    }

    pub fn from_contraction(pos: Vector4<f32>, qmatrix: Matrix4<f32>) -> Self {
        Self {
            pos,
            qmatrix,
            link: None
        }
    }

    pub fn establish_link(&mut self, other: &'a QuadricVertex<'a>) {
        self.link = Some(other);
    }

    pub fn compute_and_store_qmatrix(&mut self, shared_triangles: &[&QuadricTriangle]) {

        let mut matrix_sum: Matrix4<f32> = Matrix4::zero();

        // Loop over all planes in the list
        for st in shared_triangles {

            // Extract the plane this triangle lies in
            let p = st.construct_plane();

            // Calculate ppT
            let a = p.x;
            let b = p.y;
            let c = p.z;
            let d = p.w;
            let m = Matrix4::new( a*a, a*b, a*c, a*d,
                                                a*b, b*b, b*c, b*d,
                                                a*c, b*c, c*c, c*d,
                                                a*d, b*d, c*d, d*d);

            // Add this matrix to the sum
            matrix_sum += m;
        }

        // The sum is now the Qmatrix for this vertex, set the new qmatrix
        self.qmatrix = matrix_sum;

    }

    pub fn compute_error(&self) -> f32 {
        // The error at vertex v is defined as vT (Q v)
        let rhv = self.qmatrix * self.pos;
        self.pos.dot(rhv)
    }

    pub fn is_valid_against(&self, other: &QuadricVertex) {

    }

    pub fn contract_with(&mut self, other: &mut QuadricVertex) {

    }

}


impl<'a> PartialEq for QuadricVertex<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.qmatrix == other.qmatrix && self.link == other.link
    }
}



pub struct QuadricTriangle<'a> {
    pub p1: &'a QuadricVertex<'a>,
    pub p2: &'a QuadricVertex<'a>,
    pub p3: &'a QuadricVertex<'a>
}

impl<'a> QuadricTriangle<'a> {

    pub fn new(p1: &'a QuadricVertex, p2: &'a QuadricVertex, p3: &'a QuadricVertex) -> Self {
        Self {
            p1,
            p2,
            p3
        }
    }

    pub fn contains(&self, other: &'a QuadricVertex) -> bool {
        self.p1 == other || self.p2 == other || self.p3 == other
    }

    pub fn construct_plane(&self) -> Vector4<f32> {

        // Take two linearly independent vectors from the triangle (1-2) and (1-3)
        let v1 = self.p2.pos - self.p1.pos;
        let v2 = self.p3.pos - self.p1.pos;

        // The plane normal is the cross product between v1 and v2
        let normal = Vector4 {
            x: v2.y * v1.z - v2.z * v1.y,
            y: v2.z * v1.x - v2.x * v1.z,
            z: v2.x * v1.y - v2.y * v1.x,
            w: 0.0
        };

        // Since a normal is always of unit-length, normalise the normal
        normal.normalize();

        // Calculate the distance from the origin, along the normal, to the plane
        // This is done through the dot product of the normal and one of the vectors in the triangle
        let dist_origin = normal.dot(self.p1.pos);

        // The plane can now be constructed from this normal and the distance to the origin
        Vector4 {
            x: normal.x,
            y: normal.y,
            z: normal.z,
            w: dist_origin
        }

    }

}



// A QuadricVertexPair describes a pair of vertices that may be contractex:uture
pub struct QuadricVertexPair<'a> {
    pub left: &'a QuadricVertex<'a>,
    pub right: &'a QuadricVertex<'a>
}

impl<'a> QuadricVertexPair<'a> {

    fn compute_optimal_midpoint(&self) -> Vector4<f32> {
        (self.left.pos + self.right.pos) / 2.0
    }

    pub fn cost(&self) -> f32 {

        // v is the optimal midpoint for the contraction of these two vertices
        let v = self.compute_optimal_midpoint();

        // The cost is computed as vT (Q1 + Q1) v
        let rhv = (self.left.qmatrix + self.right.qmatrix) * v;

        // Compute vT rhv
        v.dot(rhv)

    }

    pub fn contract(&self) -> QuadricVertex {

        // Compute the new qmatrix by adding them
        let qmatrix = self.left.qmatrix + self.right.qmatrix;

        // Select a new vertex position
        let pos = self.compute_optimal_midpoint();

        // Create the new vertex
        let result = QuadricVertex::from_contraction(pos, qmatrix);

        // Linking cannot be done in the scope of a pair, since both vertices are immutable.
        // Return the resulting vertex
        result

    }

}