import HarmoniusRendering
internal import HarmoniusShaderTypes
import Testing

private enum TriangleGeometryFixtures {
  static let radius: Float = 240
  static var red: HarmoniusFloat4 { color(red: 1, green: 0, blue: 0, alpha: 1) }
  static var green: HarmoniusFloat4 { color(red: 0, green: 1, blue: 0, alpha: 1) }
  static var blue: HarmoniusFloat4 { color(red: 0, green: 0, blue: 1, alpha: 1) }

  private static func color(
    red: Float,
    green: Float,
    blue: Float,
    alpha: Float
  ) -> HarmoniusFloat4 {
    HarmoniusFloat4(x: red, y: green, z: blue, w: alpha)
  }
}

private func colorsEqual(_ lhs: HarmoniusFloat4, _ rhs: HarmoniusFloat4) -> Bool {
  lhs.x == rhs.x
    && lhs.y == rhs.y
    && lhs.z == rhs.z
    && lhs.w == rhs.w
}

private func length(_ value: HarmoniusFloat2) -> Float {
  sqrtf(value.x * value.x + value.y * value.y)
}

private func distance(_ lhs: HarmoniusFloat2, _ rhs: HarmoniusFloat2) -> Float {
  let x = lhs.x - rhs.x
  let y = lhs.y - rhs.y
  return sqrtf(x * x + y * y)
}

@Test func maxFramesInFlightIsThree() {
  #expect(TriangleVertexLayout.maxFramesInFlight == 3)
}

@Test func frameDataUsesPrimaryColors() {
  let frame = TriangleGeometry.frameData()

  #expect(colorsEqual(frame.vertex0.color, TriangleGeometryFixtures.red))
  #expect(colorsEqual(frame.vertex1.color, TriangleGeometryFixtures.green))
  #expect(colorsEqual(frame.vertex2.color, TriangleGeometryFixtures.blue))
}

@Test func frameDataPlacesVerticesOnCircle() {
  let frame = TriangleGeometry.frameData()
  let radius = TriangleGeometryFixtures.radius
  let tolerance: Float = 0.001

  for vertex in [frame.vertex0, frame.vertex1, frame.vertex2] {
    let actualDistance = length(vertex.position)
    #expect(abs(actualDistance - radius) < tolerance)
  }
}

@Test func frameDataVerticesAreEquilateral() {
  let frame = TriangleGeometry.frameData()
  let tolerance: Float = 0.001

  let side01 = distance(frame.vertex0.position, frame.vertex1.position)
  let side12 = distance(frame.vertex1.position, frame.vertex2.position)
  let side20 = distance(frame.vertex2.position, frame.vertex0.position)

  #expect(abs(side01 - side12) < tolerance)
  #expect(abs(side12 - side20) < tolerance)
}
