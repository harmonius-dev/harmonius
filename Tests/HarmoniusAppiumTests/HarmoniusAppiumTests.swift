import Foundation
import WebDriver
import XCTest

#if canImport(FoundationNetworking)
  import FoundationNetworking
#endif

private enum AppiumPlatform: String {
  case iOSSimulator = "ios-simulator"
  case macOS = "macos"
}

private enum AppiumTestError: Error, CustomStringConvertible {
  case missingEnvironment(String)
  case unsupportedPlatform(String)

  var description: String {
    switch self {
    case .missingEnvironment(let name):
      return "missing required environment variable \(name)"
    case .unsupportedPlatform(let platform):
      return "unsupported Appium platform \(platform)"
    }
  }
}

extension ElementLocator {
  fileprivate static func accessibilityID(_ value: String) -> Self {
    Self(using: "accessibility id", value: value)
  }
}

final class HarmoniusAppiumTests: XCTestCase {
  func testHarmoniusLaunchesAndExposesMainContent() throws {
    let session = try makeSession()
    defer { try? session.delete() }

    let content = try session.findElement(
      locator: .accessibilityID("snapshot-content"),
      waitTimeout: 15
    )
    XCTAssertTrue(try content.displayed)
    XCTAssertTrue(isPNG(data: try session.screenshot()))
  }

  private func makeSession() throws -> Session {
    let serverURL = try environmentURL("APPIUM_SERVER_URL")
    let platformName = try environment("HARMONIUS_APPIUM_PLATFORM")
    guard let platform = AppiumPlatform(rawValue: platformName) else {
      throw AppiumTestError.unsupportedPlatform(platformName)
    }

    let webDriver = AppiumHTTPWebDriver(
      endpoint: serverURL,
      requestTimeout: 240
    )
    let appPath = try environment("HARMONIUS_APPIUM_APP")
    var capabilities = AppiumCapabilities(
      automationName: platform.automationName,
      newCommandTimeout: 120,
      platformName: platform.w3cPlatformName,
      printPageSourceOnFindFailure: true
    )

    switch platform {
    case .macOS:
      capabilities.appPath = appPath
      capabilities.arguments = ["-HarmoniusSnapshotMode"]
      capabilities.bundleId = "dev.harmonius.App"
    case .iOSSimulator:
      capabilities.app = appPath
      capabilities.deviceName =
        ProcessInfo.processInfo.environment[
          "HARMONIUS_APPIUM_DEVICE"
        ] ?? "iPhone 17"
      capabilities.platformVersion =
        ProcessInfo.processInfo.environment[
          "HARMONIUS_APPIUM_IOS_VERSION"
        ]
      capabilities.processArguments = AppiumProcessArguments(
        args: ["-HarmoniusSnapshotMode"],
        env: [:]
      )
      capabilities.wdaLaunchTimeout = 180_000
      capabilities.wdaStartupRetries = 3
    }

    let response = try webDriver.send(
      CreateAppiumSession(capabilities: capabilities)
    )
    let session = Session(
      webDriver: webDriver,
      existingId: response.value.sessionId,
      owned: true
    )
    session.implicitWaitTimeout = 5
    return session
  }
}

extension AppiumPlatform {
  fileprivate var automationName: String {
    switch self {
    case .macOS:
      return "mac2"
    case .iOSSimulator:
      return "XCUITest"
    }
  }

  fileprivate var w3cPlatformName: String {
    switch self {
    case .macOS:
      return "mac"
    case .iOSSimulator:
      return "iOS"
    }
  }
}

private struct CreateAppiumSession: Request {
  let capabilities: AppiumCapabilities

  var pathComponents: [String] { ["session"] }
  var method: HTTPMethod { .post }
  var body: Body {
    Body(
      capabilities: CapabilityMatch(
        alwaysMatch: capabilities,
        firstMatch: nil
      )
    )
  }

  typealias Response = Requests.ResponseWithValue<ResponseValue>

  struct Body: Codable {
    let capabilities: CapabilityMatch
  }

  struct CapabilityMatch: Codable {
    let alwaysMatch: AppiumCapabilities
    let firstMatch: [AppiumCapabilities]?
  }

  struct ResponseValue: Codable {
    let sessionId: String
  }
}

private struct AppiumCapabilities: Codable {
  var app: String?
  var appPath: String?
  var arguments: [String]?
  var automationName: String?
  var bundleId: String?
  var deviceName: String?
  var newCommandTimeout: Int?
  var platformName: String?
  var platformVersion: String?
  var printPageSourceOnFindFailure: Bool?
  var processArguments: AppiumProcessArguments?
  var wdaLaunchTimeout: Int?
  var wdaStartupRetries: Int?

  enum CodingKeys: String, CodingKey {
    case app = "appium:app"
    case appPath = "appium:appPath"
    case arguments = "appium:arguments"
    case automationName = "appium:automationName"
    case bundleId = "appium:bundleId"
    case deviceName = "appium:deviceName"
    case newCommandTimeout = "appium:newCommandTimeout"
    case platformName
    case platformVersion = "appium:platformVersion"
    case printPageSourceOnFindFailure = "appium:printPageSourceOnFindFailure"
    case processArguments = "appium:processArguments"
    case wdaLaunchTimeout = "appium:wdaLaunchTimeout"
    case wdaStartupRetries = "appium:wdaStartupRetries"
  }
}

private struct AppiumProcessArguments: Codable {
  let args: [String]
  let env: [String: String]
}

private struct AppiumHTTPWebDriver: WebDriver {
  let endpoint: URL
  let requestTimeout: TimeInterval
  let wireProtocol: WireProtocol = .w3c

  func send<Req: Request>(_ request: Req) throws -> Req.Response {
    var urlRequest = try buildURLRequest(for: request)
    urlRequest.timeoutInterval = requestTimeout

    let (status, responseData) = try send(urlRequest)
    guard status == 200 else {
      throw try JSONDecoder().decode(ErrorResponse.self, from: responseData)
    }
    return try JSONDecoder().decode(Req.Response.self, from: responseData)
  }

  private func buildURLRequest<Req: Request>(
    for request: Req
  ) throws -> URLRequest {
    var url = endpoint
    for (index, pathComponent) in request.pathComponents.enumerated() {
      let last = index == request.pathComponents.count - 1
      url.appendPathComponent(pathComponent, isDirectory: !last)
    }

    var urlRequest = URLRequest(url: url)
    urlRequest.httpMethod = request.method.rawValue
    if Req.Body.self != CodableNone.self {
      urlRequest.addValue(
        "application/json;charset=UTF-8",
        forHTTPHeaderField: "content-type"
      )
      urlRequest.httpBody = try JSONEncoder().encode(request.body)
    }
    return urlRequest
  }

  private func send(_ request: URLRequest) throws -> (Int, Data) {
    let responseBox = AppiumHTTPResponse()
    let semaphore = DispatchSemaphore(value: 0)
    let task = URLSession.shared.dataTask(with: request) { data, urlResponse, error in
      if let error {
        responseBox.store(.failure(error))
      } else if let data, let urlResponse = urlResponse as? HTTPURLResponse {
        responseBox.store(.success((data, urlResponse)))
      } else {
        responseBox.store(
          .failure(
            NSError(domain: NSURLErrorDomain, code: URLError.unknown.rawValue)
          )
        )
      }
      semaphore.signal()
    }
    task.resume()
    semaphore.wait()

    switch responseBox.load() {
    case .failure(let error):
      throw error
    case .success((let data, let response)):
      return (response.statusCode, data)
    }
  }
}

private final class AppiumHTTPResponse: @unchecked Sendable {
  private let lock = NSLock()
  private var result: Result<(Data, HTTPURLResponse), Error> =
    .failure(NSError(domain: NSURLErrorDomain, code: URLError.unknown.rawValue))

  func store(_ newResult: Result<(Data, HTTPURLResponse), Error>) {
    lock.lock()
    result = newResult
    lock.unlock()
  }

  func load() -> Result<(Data, HTTPURLResponse), Error> {
    lock.lock()
    defer { lock.unlock() }
    return result
  }
}

private func environment(_ name: String) throws -> String {
  guard let value = ProcessInfo.processInfo.environment[name], !value.isEmpty else {
    throw AppiumTestError.missingEnvironment(name)
  }
  return value
}

private func environmentURL(_ name: String) throws -> URL {
  let value = try environment(name)
  guard let url = URL(string: value) else {
    throw AppiumTestError.missingEnvironment(name)
  }
  return url
}

private func isPNG(data: Data) -> Bool {
  let signature: [UInt8] = [137, 80, 78, 71, 13, 10, 26, 10]
  return data.starts(with: signature)
}
