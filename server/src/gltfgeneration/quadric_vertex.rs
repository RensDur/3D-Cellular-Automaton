use cgmath::{Vector4, Matrix4, Zero, InnerSpace};




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

    pub fn compute_qmatrix(&mut self, planes: &[Vector4<f32>]) {

        let mut matrix_sum: Matrix4<f32> = Matrix4::zero();

        // Loop over all planes in the list
        for p in planes {
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




// A QuadricVertexPair describes a pair of vertices that may be contracted in the future
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