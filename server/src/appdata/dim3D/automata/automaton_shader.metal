#include <metal_stdlib>

using namespace metal;

kernel
void
compute_iteration(
    float*                          args        [[buffer(0)]],
    texture3d<uint, access::read>   inTexture   [[texture(1)]],
    texture3d<uint, access::write>  outTexture  [[texture(2)]],
    uint4                           gid         [[thread_position_in_grid]])
{

    

}