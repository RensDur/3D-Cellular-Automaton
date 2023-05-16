#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint8_t *data;
    volatile device uint8_t *sum;
    device uint* arg_size_container;
    device float* arg_chemicals;
    device int* arg_dc_neighbours;
    device int* arg_uc_neighbours;
};

kernel void compute_iteration(device SumInput& input [[ buffer(0) ]],
                uint ugid [[ thread_position_in_grid ]])
{

    int gid = ugid;

    uint size = input.arg_size_container[0];
    uint array_size = size*size*size;
    int array_size_i = (int) array_size;
    uint dc_neighbours_len = input.arg_size_container[1];
    uint uc_neighbours_len = input.arg_size_container[2];

    float dc_range = input.arg_chemicals[0];
    float dc_influence = input.arg_chemicals[1];
    float uc_range = input.arg_chemicals[2];
    float uc_influence = input.arg_chemicals[3];

    // Calculate the influence of neighbours on this voxel
    float influence_sum = 0.0;

    for (uint i = 0; i < dc_neighbours_len; i++) {
        int index = gid + input.arg_dc_neighbours[i];

        if (index >= array_size_i) {
            index = index - array_size_i;
        }

        if (index < 0) {
            index = array_size_i + index;
        }

        if (input.data[index] == 0) {
            // DC
            influence_sum += dc_influence;
            influence_sum -= uc_influence;
        }
    }

    for (uint i = 0; i < uc_neighbours_len; i++) {
        int index = gid + input.arg_uc_neighbours[i];

        if (index >= array_size_i) {
            index = index - array_size_i;
        }

        if (index < 0) {
            index = array_size_i + index;
        }

        if (input.data[index] == 0) {
            // UC
            influence_sum += uc_influence;
        }
    }

    // PRODUCTION CODE
    if (influence_sum > 0.0) {
        input.sum[gid] = 0;
    } else if (influence_sum < 0.0) {
        input.sum[gid] = 1;
    } else {
        input.sum[gid] = input.data[gid];
    }



    // DEBUGGING CODE: CHECK INDEX COVERAGE
    //input.sum[gid] = 0;


    // DEBUGGING CODE: NEIGHBOURS SHOULD CRAWL AROUND BORDERS
    // if (gid == 12 + 25*50 + 25*50*50 || gid == 0) {
    //     for (uint i = 0; i < dc_neighbours_len; i++) {
    //         int index = (gid + input.arg_dc_neighbours[i]);

    //         if (index >= array_size_i) {
    //             index = index - array_size_i;
    //         }

    //         if (index < 0) {
    //             index = array_size_i + index;
    //         }

    //         input.sum[index] = 1;
    //     }
    // }

    // if (gid == 38 + 25*50 + 25*50*50 || gid == 49 + 49*50 + 49*50*50) {
    //     for (uint i = 0; i < uc_neighbours_len; i++) {
    //         int index = (gid + input.arg_uc_neighbours[i]);

    //         if (index >= array_size_i) {
    //             index = index - array_size_i;
    //         }

    //         if (index < 0) {
    //             index = array_size_i + index;
    //         }

    //         input.sum[index] = 1;
    //     }
    // }
    

}
