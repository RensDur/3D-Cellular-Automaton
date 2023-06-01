#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint8_t *data;
    volatile device uint8_t *sum;
    device uint* arg_size_container;
    device float* arg_chemicals;
    device int* arg_neighbours_promote_x;
    device int* arg_neighbours_promote_y;
    device int* arg_neighbours_promote_z;
    device int* arg_neighbours_demote_x;
    device int* arg_neighbours_demote_y;
    device int* arg_neighbours_demote_z;
};

kernel void compute_iteration(device SumInput& input [[ buffer(0) ]],
                uint ugid [[ thread_position_in_grid ]])
{

    int gid = ugid;

    // Structure of the size_container:
    // [automaton size,     #chemicals,         chemical_group0.#neighbours_promote, chemical_group0.#neighbours_promote, ... ]

    uint size = input.arg_size_container[0];
    int size_i = (int) size;

    uint array_size = size*size*size;
    int array_size_i = (int) array_size;

    // Extract the number of chemicals in this simulation from the size_container (structure specified above)
    int num_chemicals = (int) input.arg_size_container[1];

    // Find x, y, z coordinates by using modulo calculations
    int z = gid / (size_i*size_i);
    int y = (gid % (size_i*size_i)) / size_i;
    int x = (gid % (size_i*size_i)) % size_i;

    //
    // Influences are organised in the arg_chemicals array as follows:
    // [chemical_group0.promote.influence, chemical_group0.demote.influence, ...]

    // Calculate the influence of neighbours on this voxel.
    // There will be as many influences as there are chemical groups
    float influences[num_chemicals] = { 0.0 };

    

    // Loop over every chemical group
    for (int i = 0; i < num_chemicals; i++) {

        // Extract the influence of each chemical in this group
        float promotor_influence = input.arg_chemicals[i*2];
        float demotor_influence = input.arg_chemicals[i*2 + 1];

        // Extract the number of promotor- and demotor-neighbours for this group
        int num_promotor_chemicals = input.arg_size_container[2 + (i*2)];
        int num_demotor_chemicals = input.arg_size_container[2 + (i*2 + 1)];


        // Loop over all promotor neighbours for this chemical
        




        // Loop over all demotor neighbours for this chemical



    }





    

}
