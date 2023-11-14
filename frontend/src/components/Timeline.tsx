import clsx from "clsx"
import {formatSeconds, getSegmentColor, getSegmentTextColor} from "../helpers"
import React, {Ref, RefObject, useMemo, useRef} from "react"
import * as d3 from "d3"
import useContainerSize from "@/hooks/useContainerSize"

interface Item {
  label: string
  length: number
  offset: number
}

interface Props {
  items: Item[]
  length: number
  className?: string
  onItemClick: (item: Item, index: number) => void
  selectedIndex?: number
  fadeInactiveItems?: boolean
  time?: number
  markPoints?: number[]
  onMarkerClick?: (time: number, e: React.MouseEvent) => void
  onTimelineClick?: (time: number, e: React.MouseEvent) => void
}

const marginLeft = 4

interface TimeAxisProps {
  length: number
  onClick: (e: React.MouseEvent) => void
}

const TimeAxis = ({length, onClick}: TimeAxisProps) => {
  const svgRef = useRef<SVGSVGElement>(null)
  // @ts-expect-error this type definition is too strict, works fine
  const {width} = useContainerSize(svgRef)
  const range = [0, width]
  const domain = [0, length]
  const domainDep = domain.join(",")
  const rangeDep = range.join(",")

  const ticks = useMemo(() => {
    const xScale = d3.scaleLinear().domain(domain).range(range)
    const pixelsPerTick = 100
    const numberOfTicksTarget = Math.max(1, Math.floor(width / pixelsPerTick))
    return xScale.ticks(numberOfTicksTarget).map((value) => ({
      value,
      xOffset: xScale(value),
    }))
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [domainDep, rangeDep, width])

  return (
    <svg
      onClick={onClick}
      ref={svgRef}
      height={22}
      className="w-full z-20 cursor-pointer select-none"
    >
      <path
        d={["M", range[0] + marginLeft, 6, "v", -6, "H", range[1], "v", 6].join(
          " ",
        )}
        fill="none"
        stroke="currentColor"
      />
      {ticks.map(({value, xOffset}) => (
        <g key={value} transform={`translate(${xOffset + marginLeft}, 0)`}>
          <line y2="6" stroke="currentColor" />
          <text
            key={value}
            style={{
              fontSize: "12px",
              textAnchor: "middle",
              transform: "translateY(20px)",
            }}
          >
            {value === 0 ? "0" : formatSeconds(value, "short")}
          </text>
        </g>
      ))}
    </svg>
  )
}

type TimelineSegmentsProps = Pick<
  Props,
  "items" | "selectedIndex" | "onItemClick" | "fadeInactiveItems" | "length"
>

const TimelineSegments: React.FC<TimelineSegmentsProps> = ({
  items,
  selectedIndex,
  onItemClick,
  fadeInactiveItems,
  length,
}) => {
  const styles = useMemo(() => {
    return items.map((item, index) => {
      const backgroundColor = getSegmentColor(index, items.length)
      const textColor = getSegmentTextColor(backgroundColor)
      const widthPercent = (item.length / length) * 100
      const offset = (item.offset / length) * 100
      return {
        backgroundColor,
        color: textColor,
        width: `${widthPercent}%`,
        left: `calc(${offset}%)`,
        display: "absolute",
      } satisfies React.CSSProperties
    })
  }, [items, length])

  return items.map((item, index) => {
    const style = styles[index]
    return (
      <div
        key={index}
        className={clsx(
          "absolute text-sm cursor-pointer text-white text-center py-2 truncate",
          !fadeInactiveItems && "hover:opacity-80",
          index !== selectedIndex &&
            fadeInactiveItems &&
            "opacity-30 hover:opacity-60",
          index === selectedIndex && fadeInactiveItems && "opacity-100",
        )}
        style={style}
        onClick={() => onItemClick(item, index)}
      >
        {item.label}
      </div>
    )
  })
}

const Timeline: React.FC<Props> = ({
  items,
  onItemClick,
  selectedIndex,
  fadeInactiveItems,
  length,
  time,
  markPoints,
  onMarkerClick,
  onTimelineClick,
  className,
}) => {
  const playheadPosition =
    typeof time === "number"
      ? `calc(${(time / length) * 100}% - (1.25rem / 2))`
      : undefined

  const handleTimelineClick = (e: React.MouseEvent) => {
    if (onTimelineClick) {
      const rect = e.currentTarget.getBoundingClientRect()
      const x = e.clientX - rect.left
      const time = (x / rect.width) * length
      onTimelineClick(time, e)
    }
  }

  return (
    <section className={className}>
      <div
        className="flex h-[36px] relative w-full bg-base-200"
        style={{marginLeft}}
      >
        {typeof time === "number" && (
          <span
            style={{left: playheadPosition}}
            className="absolute bottom-[-10px] bg-gray-700 rounded-full w-5 h-5 z-10 border-2 border-gray-400"
          />
        )}
        {markPoints?.map((time) => (
          <span
            key={time}
            style={{left: `calc(${(time / length) * 100}% + ${marginLeft}px)`}}
            className="top-0 absolute py-2 bg-green-500 w-2 -translate-x-0.5 bg-opacity-50 h-[36px] z-10 cursor-pointer"
            onClick={(e) => onMarkerClick && onMarkerClick(time, e)}
          />
        ))}
        <TimelineSegments
          items={items}
          onItemClick={onItemClick}
          selectedIndex={selectedIndex}
          fadeInactiveItems={fadeInactiveItems}
          length={length}
        />
      </div>
      <TimeAxis onClick={handleTimelineClick} length={length} />
    </section>
  )
}

export default Timeline
