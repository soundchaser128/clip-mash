import {
  parseTimestamp,
  formatSeconds,
  sumDurations,
  HasDuration,
} from "../helpers/time"
import {describe, expect, it} from "vitest"

describe("parseTimestamp", () => {
  it("should be able to parse timestamps like 01:20", () => {
    expect(parseTimestamp("01:20")).toBe(80)
    expect(parseTimestamp("00:00")).toBe(0)
    expect(parseTimestamp("1:40")).toBe(100)
    expect(parseTimestamp("00:30")).toBe(30)
    expect(parseTimestamp("00:00:30")).toBe(30)
    expect(parseTimestamp("00:00:00")).toBe(0)
    expect(parseTimestamp("00:00:01")).toBe(1)
    expect(parseTimestamp("00:01:00")).toBe(60)
    expect(parseTimestamp("00:01:01.500")).toBe(61.5)
  })
})

describe("formatSeconds", () => {
  it("should format hours/seconds in short format", () => {
    expect(formatSeconds(50, "short")).toBe("00:50")
    expect(formatSeconds(200, "short")).toBe("03:20")
    expect(formatSeconds(100.08, "short")).toBe("01:40")
    expect(formatSeconds(0, "short")).toBe("00:00")
    expect(formatSeconds(60 * 60 * 2 + 10, "short")).toBe("02:00:10")
    expect(formatSeconds(0.5, "short-with-ms")).toBe("00:00.500")
    expect(formatSeconds(0.5, "short")).toBe("00:00")
    expect(formatSeconds(60 * 60 * 2 + 10.5, "short-with-ms")).toBe(
      "02:00:10.500",
    )
  })

  it("should format hours/seconds in long format", () => {
    expect(formatSeconds(50, "long")).toBe("50 seconds")
    expect(formatSeconds(200, "long")).toBe("3 minutes 20 seconds")
    expect(formatSeconds(100.08, "long")).toBe("1 minute 40 seconds")
    expect(formatSeconds(0, "long")).toBe("0 seconds")
  })
})

describe("sumDurations", () => {
  it("should return 0 when no markers are provided", () => {
    expect(sumDurations()).toBe(0)
  })

  it("should return 0 when no markers are selected", () => {
    const markers = [
      {selected: false, selectedRange: [0, 10], loops: 1},
      {selected: false, selectedRange: [10, 20], loops: 1},
      {selected: false, selectedRange: [20, 30], loops: 1},
    ] satisfies HasDuration[]
    expect(sumDurations(markers)).toBe(0)
  })

  it("should return the sum of selected marker durations", () => {
    const markers = [
      {selected: true, selectedRange: [0, 10], loops: 1},
      {selected: false, selectedRange: [10, 20], loops: 1},
      {selected: true, selectedRange: [20, 30], loops: 1},
    ] satisfies HasDuration[]
    expect(sumDurations(markers)).toBe(20)
  })

  it("should handle markers with overlapping selected ranges", () => {
    const markers = [
      {selected: true, selectedRange: [0, 10], loops: 1},
      {selected: true, selectedRange: [5, 15], loops: 1},
      {selected: true, selectedRange: [10, 20], loops: 1},
    ] satisfies HasDuration[]
    expect(sumDurations(markers)).toBe(30)
  })

  it("should take loops into account", () => {
    const markers = [
      {selected: true, selectedRange: [0, 10], loops: 1},
      {selected: true, selectedRange: [10, 20], loops: 2},
      {selected: true, selectedRange: [20, 30], loops: 3},
    ] satisfies HasDuration[]
    expect(sumDurations(markers)).toBe(60)
  })
})
