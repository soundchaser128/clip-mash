import {parseTimestamp} from "../helpers"
import {describe, expect, it} from "vitest"

describe("parseTimestamp", () => {
  it("should be able to parse timestamps like 01:20", () => {
    expect(parseTimestamp('01:20')).toBe(80)
  })
})
