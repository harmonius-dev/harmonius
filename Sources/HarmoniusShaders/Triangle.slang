struct VertexData {
  float2 position;
  float4 color;
};

struct TriangleData {
  VertexData vertex0;
  VertexData vertex1;
  VertexData vertex2;
};

struct RasterizerData {
  float4 position : SV_Position;
  float4 color    : COLOR0;
};

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
  float shortSide = min(viewport.x, viewport.y);
  out.position.xy = pixelSpacePosition / (shortSide * 0.5);
  out.position.z = 0.0;
  out.position.w = 1.0;
  out.color = vertexData[vertexID].color;
  return out;
}

[shader("fragment")]
float4 fragmentShader(RasterizerData input) : SV_Target {
  return input.color;
}
