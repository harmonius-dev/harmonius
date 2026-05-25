#ifdef __METAL_VERSION__
#include <metal_stdlib>
using namespace metal;
#else
#include <simd/simd.h>
#endif

#include "NSEnum.h"

#ifdef __METAL_VERSION__
NS_ENUM(uint32_t, InputBufferIndex) {
#else
typedef NS_ENUM(uint32_t, InputBufferIndex) {
#endif
  InputBufferIndexForVertexData = 0,
  InputBufferIndexForViewportSize = 1,
};

typedef struct {
  simd_float2 position;
  simd_float4 color;
} VertexData;

typedef struct {
  VertexData vertex0;
  VertexData vertex1;
  VertexData vertex2;
} TriangleData;
