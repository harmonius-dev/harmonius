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
    let red = color(red: 1, green: 0, blue: 0, alpha: 1)
    let green = color(red: 0, green: 1, blue: 0, alpha: 1)
    let blue = color(red: 0, green: 0, blue: 1, alpha: 1)

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
    var value = hlslpp.float2()
    value[0] = radius * cosf(angle)
    value[1] = radius * sinf(angle)
    return value
  }

  private static func color(
    red: Float,
    green: Float,
    blue: Float,
    alpha: Float
  ) -> hlslpp.float4 {
    var value = hlslpp.float4()
    value[0] = red
    value[1] = green
    value[2] = blue
    value[3] = alpha
    return value
  }
}
