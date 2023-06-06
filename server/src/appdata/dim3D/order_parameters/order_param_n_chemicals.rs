use crate::appdata::dim3d::automata::automaton_gpu_n_chemicals::GPUNChemicalsCellularAutomaton3D;
use super::order_param::OrderParameter;

impl OrderParameter for GPUNChemicalsCellularAutomaton3D {

    fn compute_order_parameter(&self) -> f32 {

        0.0

    }

}