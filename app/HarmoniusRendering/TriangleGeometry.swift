import Foundation
import simd

enum TriangleVertexLayout {
  static let maxFramesInFlight = 3
}

struct TriangleVertexData {
  var position: SIMD2<Float>
  var color: SIMD4<Float>
}

struct TriangleFrameData {
  var vertex0: TriangleVertexData
  var vertex1: TriangleVertexData
  var vertex2: TriangleVertexData
}

enum TriangleGeometry {
  private static let red = SIMD4<Float>(1, 0, 0, 1)
  private static let green = SIMD4<Float>(0, 1, 0, 1)
  private static let blue = SIMD4<Float>(0, 0, 1, 1)

  static func frameData(rotationDegrees: Int) -> TriangleFrameData {
    let radius: Float = 350
    let angle0 = Float(rotationDegrees) * .pi / 180
    let angle1 = angle0 + (2 * .pi / 3)
    let angle2 = angle0 + (4 * .pi / 3)

    return TriangleFrameData(
      vertex0: TriangleVertexData(
        position: SIMD2(radius * cos(angle0), radius * sin(angle0)),
        color: red
      ),
      vertex1: TriangleVertexData(
        position: SIMD2(radius * cos(angle1), radius * sin(angle1)),
        color: green
      ),
      vertex2: TriangleVertexData(
        position: SIMD2(radius * cos(angle2), radius * sin(angle2)),
        color: blue
      )
    )
  }
}
