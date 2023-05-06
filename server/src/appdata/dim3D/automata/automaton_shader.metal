#include <metal_stdlib>

using namespace metal;

kernel void compute_iteration(  texture3d<uint, access::read>   inTexture   [[texture(AAPLTextureIndexInput)]],
                                texture3d<uint, access::write>  outTexture  [[texture(AAPLTextureIndexOutput)]],
                                uint3                           gid         [[thread_position_in_grid]])
{

    

}