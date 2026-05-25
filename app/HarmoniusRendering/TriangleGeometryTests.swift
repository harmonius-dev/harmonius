import HarmoniusRendering
import Testing
import simd

private enum TriangleGeometryFixtures {
  static let radius: Float = 350
  static let red = SIMD4<Float>(1, 0, 0, 1)
  static let green = SIMD4<Float>(0, 1, 0, 1)
  static let blue = SIMD4<Float>(0, 0, 1, 1)
}

@Test func maxFramesInFlightIsThree() {
  #expect(TriangleVertexLayout.maxFramesInFlight == 3)
}

@Test func frameDataUsesPrimaryColors() {
  let frame = TriangleGeometry.frameData()

  #expect(frame.vertex0.color == TriangleGeometryFixtures.red)
  #expect(frame.vertex1.color == TriangleGeometryFixtures.green)
  #expect(frame.vertex2.color == TriangleGeometryFixtures.blue)
}

@Test func frameDataPlacesVerticesOnCircle() {
  let frame = TriangleGeometry.frameData()
  let radius = TriangleGeometryFixtures.radius
  let tolerance: Float = 0.001

  for vertex in [frame.vertex0, frame.vertex1, frame.vertex2] {
    let distance = simd_length(vertex.position)
    #expect(abs(distance - radius) < tolerance)
  }
}

@Test func frameDataVerticesAreEquilateral() {
  let frame = TriangleGeometry.frameData()
  let tolerance: Float = 0.001

  let side01 = simd_distance(frame.vertex0.position, frame.vertex1.position)
  let side12 = simd_distance(frame.vertex1.position, frame.vertex2.position)
  let side20 = simd_distance(frame.vertex2.position, frame.vertex0.position)

  #expect(abs(side01 - side12) < tolerance)
  #expect(abs(side12 - side20) < tolerance)
}
