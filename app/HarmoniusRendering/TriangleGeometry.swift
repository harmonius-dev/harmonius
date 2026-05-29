import Darwin
public import HarmoniusShaderTypes

public enum TriangleVertexLayout {
  public static let maxFramesInFlight = 3
}

public enum TriangleGeometry {
  public static func frameData() -> TriangleData {
    let radius: Float = 240
    let angle0: Float = 0
    let angle1 = angle0 + (2 * Float.pi / 3)
    let angle2 = angle0 + (4 * Float.pi / 3)
    let red = hlslpp.float4(SIMD4<Float>(1, 0, 0, 1))
    let green = hlslpp.float4(SIMD4<Float>(0, 1, 0, 1))
    let blue = hlslpp.float4(SIMD4<Float>(0, 0, 1, 1))

    return TriangleData(
      vertex0: VertexData(
        position: point(radius: radius, angle: angle0),
        color: red
      ),
      vertex1: VertexData(
        position: point(radius: radius, angle: angle1),
        color: green
      ),
      vertex2: VertexData(
        position: point(radius: radius, angle: angle2),
        color: blue
      )
    )
  }

  private static func point(radius: Float, angle: Float) -> hlslpp.float2 {
    hlslpp.float2(SIMD4<Float>(radius * cosf(angle), radius * sinf(angle), 0, 0))
  }
}
