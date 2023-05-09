#include <metal_stdlib>

using namespace metal;

struct
TextureInput
{
    device uint* inTex;
    volatile device uint* outTex;
};

kernel
void
compute_iteration(
    volatile device TextureInput &input [[buffer(0)]],
    uint3 gid [[thread_position_in_grid]]
)
{

    input.outTex[gid[0]] = 1;

}