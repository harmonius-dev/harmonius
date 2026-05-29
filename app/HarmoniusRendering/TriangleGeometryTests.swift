import HarmoniusRendering
internal import HarmoniusShaderTypes
import Testing

private enum TriangleGeometryFixtures {
  static let radius: Float = 240
  static var red: hlslpp.float4 { color(red: 1, green: 0, blue: 0, alpha: 1) }
  static var green: hlslpp.float4 { color(red: 0, green: 1, blue: 0, alpha: 1) }
  static var blue: hlslpp.float4 { color(red: 0, green: 0, blue: 1, alpha: 1) }

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

@Test func maxFramesInFlightIsThree() {
  #expect(TriangleVertexLayout.maxFramesInFlight == 3)
}

@Test func frameDataUsesPrimaryColors() {
  let frame = TriangleGeometry.frameData()

  #expect(hlslpp.all(frame.vertex0.color == TriangleGeometryFixtures.red))
  #expect(hlslpp.all(frame.vertex1.color == TriangleGeometryFixtures.green))
  #expect(hlslpp.all(frame.vertex2.color == TriangleGeometryFixtures.blue))
}

@Test func frameDataPlacesVerticesOnCircle() {
  let frame = TriangleGeometry.frameData()
  let radius = TriangleGeometryFixtures.radius
  let tolerance: Float = 0.001

  for vertex in [frame.vertex0, frame.vertex1, frame.vertex2] {
    let distance = hlslpp.length(vertex.position).vec.x
    #expect(abs(distance - radius) < tolerance)
  }
}

@Test func frameDataVerticesAreEquilateral() {
  let frame = TriangleGeometry.frameData()
  let tolerance: Float = 0.001

  let side01 = hlslpp.distance(frame.vertex0.position, frame.vertex1.position).vec.x
  let side12 = hlslpp.distance(frame.vertex1.position, frame.vertex2.position).vec.x
  let side20 = hlslpp.distance(frame.vertex2.position, frame.vertex0.position).vec.x

  #expect(abs(side01 - side12) < tolerance)
  #expect(abs(side12 - side20) < tolerance)
}
