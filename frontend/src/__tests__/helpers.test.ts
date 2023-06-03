import {parseTimestamp, formatSeconds} from "../helpers"
import {describe, expect, it} from "vitest"

describe("parseTimestamp", () => {
  it("should be able to parse timestamps like 01:20", () => {
    expect(parseTimestamp("01:20")).toBe(80)
    expect(parseTimestamp("00:00")).toBe(0)
    expect(parseTimestamp("1:40")).toBe(100)
    expect(parseTimestamp("00:30")).toBe(30)
  })
})

describe("formatSeconds", () => {
  it("should format hours/seconds in short format", () => {
    expect(formatSeconds(50, "short")).toBe("00:50")
    expect(formatSeconds(200, "short")).toBe("03:20")
    expect(formatSeconds(100.08, "short")).toBe("01:40")
    expect(formatSeconds(0, "short")).toBe("00:00")
  })

  it("should format hours/seconds in long format", () => {
    expect(formatSeconds(50, "long")).toBe("50 seconds")
    expect(formatSeconds(200, "long")).toBe("3 minutes 20 seconds")
    expect(formatSeconds(100.08, "long")).toBe("1 minute 40 seconds")
    expect(formatSeconds(0, "long")).toBe("0 seconds")
  })
})
