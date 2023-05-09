#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint *data;
    volatile device uint *sum;
};

kernel void sum(device SumInput& input [[ buffer(0) ]],
                uint gid [[ thread_position_in_grid ]])
{
    input.sum[gid] = 0;
}
