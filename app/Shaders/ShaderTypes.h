#include "SwiftCompat.h"

NS_ENUM(uint32_t, InputBufferIndex) {
  InputBufferIndexForVertexData = 0,
  InputBufferIndexForViewportSize = 1,
};

typedef struct {
  float2 position;
  float4 color;
} VertexData;

typedef struct {
  VertexData vertex0;
  VertexData vertex1;
  VertexData vertex2;
} TriangleData;
