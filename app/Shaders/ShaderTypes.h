#include "SwiftCompat.h"

typedef struct {
  float2 position;
  float4 color;
} VertexData;

typedef struct {
  VertexData vertex0;
  VertexData vertex1;
  VertexData vertex2;
} TriangleData;
