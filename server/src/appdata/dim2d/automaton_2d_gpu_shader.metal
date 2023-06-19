#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint8_t *data;
    volatile device uint8_t *sum;
    device uint* arg_size_container;
    device float* arg_chemicals;
    device int* arg_neighbours_promote_x;
    device int* arg_neighbours_promote_y;
    device int* arg_neighbours_demote_x;
    device int* arg_neighbours_demote_y;
};

kernel void compute_iteration(device SumInput& input [[ buffer(0) ]],
                uint ugid [[ thread_position_in_grid ]])
{

    int gid = ugid;

    // Structure of the size_container:
    // [automaton size,     #chemicals,         chemical_group0.#neighbours_promote, chemical_group0.#neighbours_promote, ... ]

    uint size = input.arg_size_container[0];
    int size_i = (int) size;

    uint array_size = size*size;
    int array_size_i = (int) array_size;

    // Extract the number of chemicals in this simulation from the size_container (structure specified above)
    int num_chemicals = (int) input.arg_size_container[1];

    // Find x, y, z coordinates by using modulo calculations
    int y = gid / size_i;
    int x = gid % size_i;

    //
    // Influences are organised in the arg_chemicals array as follows:
    // [chemical_group0.promote.influence, chemical_group0.demote.influence, ...]

    // Calculate the influence of neighbours on this voxel.
    // There will be as many influences as there are chemical groups
    float influences[10] = { 0.0 };

    

    // Loop over every chemical group
    for (int i = 0; i < num_chemicals; i++) {

        // Extract the influence of each chemical in this group
        float promotor_influence = input.arg_chemicals[i*2];
        float demotor_influence = input.arg_chemicals[i*2 + 1];

        // Extract the number of promotor- and demotor-neighbours for this group
        int num_promotor_neighbours = input.arg_size_container[2 + (i*2)];
        int num_demotor_neighbours = input.arg_size_container[2 + (i*2 + 1)];

        // We must determine where to start reading in the neighbour-arrays.
        // This starting position is determined by the total number of neighbours that was considered
        // for all chemicals that occur earlier in the array
        int sum_prev_promotor_neighbours = 0;
        int sum_prev_demotor_neighbours = 0;

        for (int j = 0; j < i; j++) {
            sum_prev_promotor_neighbours += input.arg_size_container[2 + (j*2)];
            sum_prev_demotor_neighbours += input.arg_size_container[2 + (j*2 + 1)];
        }


        // Loop over all promotor neighbours for this chemical
        for (int p = 0; p < num_promotor_neighbours; p++) {
            // Extract this neighbour's deltas
            int dx = input.arg_neighbours_promote_x[sum_prev_promotor_neighbours + p];
            int dy = input.arg_neighbours_promote_y[sum_prev_promotor_neighbours + p];

            // Determine the exact xyz position including wrapping
            int x_wrapped = x + dx;
            int y_wrapped = y + dy;

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

            // Compute the new gid by using these wrapped values
            int index = x_wrapped + y_wrapped * size_i;

            // There's one less chemical groups than there are types of pigmented cells in the environment.
            // Chemicals therefore reach from 0 to n-1 (incl).
            // A chemical group is only activated by the pigment cell they belong to.
            // Thus: if the type of cell of this neighbour matches the index of this chemical, that cell influences this one.
            if (input.data[index] == i) {
                // DC
                influences[i] += promotor_influence;
            }

        }



        // Loop over all demotor neighbours for this chemical
        for (int d = 0; d < num_demotor_neighbours; d++) {

            // Extract this neighbour's deltas
            int dx = input.arg_neighbours_demote_x[sum_prev_demotor_neighbours + d];
            int dy = input.arg_neighbours_demote_y[sum_prev_demotor_neighbours + d];

            // Determine the exact xyz position including wrapping
            int x_wrapped = x + dx;
            int y_wrapped = y + dy;

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

            // Compute the new gid by using these wrapped values
            int index = x_wrapped + y_wrapped * size_i;

            // There's one less chemical groups than there are types of pigmented cells in the environment.
            // Chemicals therefore reach from 0 to n-1 (incl).
            // A chemical group is only activated by the pigment cell they belong to.
            // Thus: if the type of cell of this neighbour matches the index of this chemical, that cell influences this one.
            if (input.data[index] == i) {
                // DC
                influences[i] += demotor_influence;
            }

        }


    }



    //
    // All influences have been computed and we can start to determine which color this
    // cell should become.
    //

    // Rules:
    // 1. Highest positive influence wins.
    // 2. If the highest influence is negative, the cell becomes undifferentiated.
    // 3. If the highest influence is exactly zero, the cell doesn't change.



    // 1. Compute the highest influence from the array of influences
    float maximum_influence = influences[0];
    int maximum_influence_owner = 0;

    for (int i = 1; i < num_chemicals; i++) {
        if (influences[i] > maximum_influence) {
            maximum_influence = influences[i];
            maximum_influence_owner = i;
        }
    }


    if (maximum_influence > 0) {
        // Apply the first rule

        // Now, we have a maximum influence that's positive.
        // The chemical-group (and thus pigment-cell type) that has won,
        // is identified by 'maximum_influence_owner';
        input.sum[gid] = maximum_influence_owner;
    } else if (maximum_influence < 0) {
        // Apply the second rule
        // Negative highest influence, this cell is undifferentiated
        input.sum[gid] = num_chemicals;
    } else {
        input.sum[gid] = input.data[gid];
    }


}
