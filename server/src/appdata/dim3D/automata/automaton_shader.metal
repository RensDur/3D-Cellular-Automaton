#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint *data;
    volatile device uint *sum;
    device uint* arg_size_container;
    device float* arg_chemicals;
    device int* arg_dc_neighbours;
    device int* arg_uc_neighbours;
};

kernel void compute_iteration(device SumInput& input [[ buffer(0) ]],
                uint gid [[ thread_position_in_grid ]])
{

    uint size = input.arg_size_container[0];
    uint dc_neighbours_len = input.arg_size_container[1];
    uint uc_neighbours_len = input.arg_size_container[2];

    float dc_range = input.arg_chemicals[0];
    float dc_influence = input.arg_chemicals[1];
    float uc_range = input.arg_chemicals[2];
    float uc_influence = input.arg_chemicals[3];

    // Calculate the influence of neighbours on this voxel
    float influence_sum = 0;

    for (uint i = 0; i < dc_neighbours_len; i++) {
        int index = gid + input.arg_dc_neighbours[i];

        if (index >= 0 && index < dc_neighbours_len) {
            if (input.data[index] == 0) {
                // DC
                influence_sum += dc_influence;
            }
        }
    }

    for (uint i = 0; i < uc_neighbours_len; i++) {
        int index = gid + input.arg_uc_neighbours[i];

        if (index >= 0 && index < uc_neighbours_len) {
            if (input.data[index] == 1) {
                // UC
                influence_sum += uc_influence;
            }
        }
    }

    if (influence_sum > 0) {
        input.sum[gid] = 0;
    } else if (influence_sum < 0) {
        input.sum[gid] = 1;
    } else {
        input.sum[gid] = input.data[gid];
    }

    //input.sum[gid] = 0;
    

}
