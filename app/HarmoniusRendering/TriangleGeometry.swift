import Foundation
public import HarmoniusShaderTypes
import simd

public enum TriangleVertexLayout {
  public static let maxFramesInFlight = 3
}

public enum TriangleGeometry {
  private static let red = SIMD4<Float>(1, 0, 0, 1)
  private static let green = SIMD4<Float>(0, 1, 0, 1)
  private static let blue = SIMD4<Float>(0, 0, 1, 1)

  public static func frameData() -> TriangleData {
    let radius: Float = 240
    let angle0: Float = 0
    let angle1 = angle0 + (2 * .pi / 3)
    let angle2 = angle0 + (4 * .pi / 3)

    return TriangleData(
      vertex0: VertexData(
        position: SIMD2(radius * cos(angle0), radius * sin(angle0)),
        color: red
      ),
      vertex1: VertexData(
        position: SIMD2(radius * cos(angle1), radius * sin(angle1)),
        color: green
      ),
      vertex2: VertexData(
        position: SIMD2(radius * cos(angle2), radius * sin(angle2)),
        color: blue
      )
    )
  }
}
