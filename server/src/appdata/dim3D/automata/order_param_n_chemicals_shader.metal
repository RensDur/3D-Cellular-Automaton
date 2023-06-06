#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint8_t *data;
    volatile device int8_t *sum;
    device uint* arg_size_container;
    device int* arg_neighbours;
};

kernel void compute_iteration(device SumInput& input [[ buffer(0) ]],
                uint ugid [[ thread_position_in_grid ]])
{

    int gid = ugid;

    // Structure of the size_container:
    // [automaton size,     #species]

    uint size = input.arg_size_container[0];
    int size_i = (int) size;

    uint array_size = size*size*size;
    int array_size_i = (int) array_size;

    // Extract the number of species in this simulation from the size_container (structure specified above)
    int num_species = (int) input.arg_size_container[1];

    // Find x, y, z coordinates by using modulo calculations
    int z = gid / (size_i*size_i);
    int y = (gid % (size_i*size_i)) / size_i;
    int x = (gid % (size_i*size_i)) % size_i;


    for (int a = 0; a < 18; a += 3) {
        int dx = input.arg_neighbours[a];
        int dy = input.arg_neighbours[a+1];
        int dz = input.arg_neighbours[a+2];

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


        for (uint8_t j = 0; j < num_species; j++) {
            for (uint8_t i = j+1; i < num_species; i++) {

                int8_t sigma1 = 0;
                int8_t sigma2 = 0;

                // Compute sigma1
                // The value of the cell that we're currently computing for is input.data[gid];
                if (input.data[gid] == i) {
                    sigma1 = 1;
                } else if (input.data[gid] == j) {
                    sigma1 = -1;
                }

                // Compute sigma2
                // The value of the neighbour we're currently considering is found at input.data[index];
                if (input.data[index] == i) {
                    sigma2 = 1;
                } else if (input.data[index] == j) {
                    sigma2 = -1;
                }

                // Add the absolute value of the multiplication of these numbers to the sum
                input.sum[gid] += sigma1 * sigma2;
                
            }
        }

    }

}
