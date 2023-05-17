#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint8_t *data;
    volatile device uint8_t *sum;
    device uint* arg_size_container;
    device float* arg_chemicals;
    device int* arg_dc_neighbours_x;
    device int* arg_dc_neighbours_y;
    device int* arg_dc_neighbours_z;
    device int* arg_uc_neighbours_x;
    device int* arg_uc_neighbours_y;
    device int* arg_uc_neighbours_z;
};

kernel void compute_iteration(device SumInput& input [[ buffer(0) ]],
                uint ugid [[ thread_position_in_grid ]])
{

    int gid = ugid;

    uint size = input.arg_size_container[0];
    int size_i = (int) size;
    uint array_size = size*size*size;
    int array_size_i = (int) array_size;
    uint dc_neighbours_len = input.arg_size_container[1];
    uint uc_neighbours_len = input.arg_size_container[2];

    int z = gid / (size_i*size_i);
    int y = (gid % (size_i*size_i)) / size_i;
    int x = (gid % (size_i*size_i)) % size_i;

    float dc_range = input.arg_chemicals[0];
    float dc_influence = input.arg_chemicals[1];
    float uc_range = input.arg_chemicals[2];
    float uc_influence = input.arg_chemicals[3];

    // Calculate the influence of neighbours on this voxel
    float influence_sum = 0.0;

    for (uint i = 0; i < dc_neighbours_len; i++) {
        int dx = input.arg_dc_neighbours_x[i];
        int dy = input.arg_dc_neighbours_y[i];
        int dz = input.arg_dc_neighbours_z[i];

        int x_wrapped = x + dx;
        int y_wrapped = y + dy;
        int z_wrapped = z + dz;

        // Wrap x, y and z around the cube individually
        // WRAPPING X
        // If smaller than zero
        if (x_wrapped < 0) {
            // Adjust so that it wraps around the cube
            x_wrapped = size_i + x_wrapped;
        }

        // If larger than or equal to array-size
        if (x_wrapped >= size_i) {
            // Adjust so that it wraps around the cube
            x_wrapped = x_wrapped - size_i;
        }

        // WRAPPING Y
        // If smaller than zero
        if (y_wrapped < 0) {
            // Adjust so that it wraps around the cube
            y_wrapped = size_i + y_wrapped;
        }

        // If larger than or equal to array-size
        if (y_wrapped >= size_i) {
            // Adjust so that it wraps around the cube
            y_wrapped = y_wrapped - size_i;
        }

        // WRAPPING Z
        // If smaller than zero
        if (z_wrapped < 0) {
            // Adjust so that it wraps around the cube
            z_wrapped = size_i + z_wrapped;
        }

        // If larger than or equal to array-size
        if (z_wrapped >= size_i) {
            // Adjust so that it wraps around the cube
            z_wrapped = z_wrapped - size_i;
        }

        // Compute the new gid by using these wrapped values
        int index = x_wrapped + y_wrapped * size_i + z_wrapped * size_i * size_i;

        if (input.data[index] == 0) {
            // DC
            influence_sum += dc_influence;
        }
    }

    for (uint i = 0; i < uc_neighbours_len; i++) {
        int dx = input.arg_uc_neighbours_x[i];
        int dy = input.arg_uc_neighbours_y[i];
        int dz = input.arg_uc_neighbours_z[i];

        int x_wrapped = x + dx;
        int y_wrapped = y + dy;
        int z_wrapped = z + dz;

        // Wrap x, y and z around the cube individually
        // WRAPPING X
        // If smaller than zero
        if (x_wrapped < 0) {
            // Adjust so that it wraps around the cube
            x_wrapped = size_i + x_wrapped;
        }

        // If larger than or equal to array-size
        if (x_wrapped >= size_i) {
            // Adjust so that it wraps around the cube
            x_wrapped = x_wrapped - size_i;
        }

        // WRAPPING Y
        // If smaller than zero
        if (y_wrapped < 0) {
            // Adjust so that it wraps around the cube
            y_wrapped = size_i + y_wrapped;
        }

        // If larger than or equal to array-size
        if (y_wrapped >= size_i) {
            // Adjust so that it wraps around the cube
            y_wrapped = y_wrapped - size_i;
        }

        // WRAPPING Z
        // If smaller than zero
        if (z_wrapped < 0) {
            // Adjust so that it wraps around the cube
            z_wrapped = size_i + z_wrapped;
        }

        // If larger than or equal to array-size
        if (z_wrapped >= size_i) {
            // Adjust so that it wraps around the cube
            z_wrapped = z_wrapped - size_i;
        }

        // Compute the new gid by using these wrapped values
        int index = x_wrapped + y_wrapped * size_i + z_wrapped * size_i * size_i;

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
    

}
