import {scaleSequential} from "d3-scale"
import {interpolatePlasma} from "d3-scale-chromatic"
import React from "react"

export function getSegmentColor(index: number, count: number): string {
  const colorScale = scaleSequential()
    .domain([0, count - 1])
    .interpolator(interpolatePlasma)

  return colorScale(index)
}

export function getSegmentTextColor(color: string): string {
  const [r, g, b] = color
    .slice(1)
    .match(/.{1,2}/g)!
    .map((x) => parseInt(x, 16))

  const luma = 0.2126 * r + 0.7152 * g + 0.0722 * b

  if (luma < 140) {
    return "#ffffff"
  } else {
    return "#000000"
  }
}

export function getSegmentStyle(
  index: number,
  count: number,
): React.CSSProperties {
  const color = getSegmentColor(index, count)
  const textColor = getSegmentTextColor(color)

  return {
    backgroundColor: color,
    color: textColor,
  }
}
