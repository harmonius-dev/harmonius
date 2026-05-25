#include "ShaderTypes.h"

struct RasterizerData {
  float4 position : SV_Position;
  float4 color    : COLOR0;
};

// Buffer indices come from ShaderTypes.h
// (InputBufferIndexForVertexData = 0, InputBufferIndexForViewportSize = 1).
// Slang for the Metal target lowers register(tN)/register(bN) into the
// shared Metal buffer table, so StructuredBuffer at t0 and ConstantBuffer
// at b1 land on buffer slots 0 and 1 respectively.
StructuredBuffer<VertexData> vertexData   : register(t0);
ConstantBuffer<uint2>        viewportSize : register(b1);

[shader("vertex")]
RasterizerData vertexShader(uint vertexID : SV_VertexID) {
  RasterizerData out;
  float2 pixelSpacePosition = vertexData[vertexID].position;
  float2 viewport = float2(viewportSize);
  out.position.xy = pixelSpacePosition / (viewport * 0.5);
  out.position.z = 0.0;
  out.position.w = 1.0;
  out.color = vertexData[vertexID].color;
  return out;
}

[shader("fragment")]
float4 fragmentShader(RasterizerData input) : SV_Target {
  return input.color;
}
