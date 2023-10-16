import clsx from "clsx"
import {getSegmentColor, getSegmentTextColor} from "../helpers"
import {useMemo, useRef} from "react"
import * as d3 from "d3"

interface Item {
  label: string
  length: number
  offset: number
}

interface Props {
  items: Item[]
  length: number
  onItemClick: (item: Item, index: number) => void
  selectedIndex?: number
  fadeInactiveItems?: boolean
  time?: number
  markPoints?: number[]
  onMarkerClick?: (time: number, index: number) => void
}

const marginLeft = 4

const TimeAxis = ({length}: {length: number}) => {
  const svgRef = useRef<SVGSVGElement>(null)
  const width = svgRef.current?.getBoundingClientRect().width || 0
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
    <svg ref={svgRef} height={22} className="w-full z-20">
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
              fontSize: "10px",
              textAnchor: "middle",
              transform: "translateY(20px)",
            }}
          >
            {value}
          </text>
        </g>
      ))}
    </svg>
  )
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
}) => {
  const styles = items.map((item, index) => {
    const backgroundColor = getSegmentColor(index, items.length)
    const textColor = getSegmentTextColor(backgroundColor)
    const widthPercent = (item.length / length) * 100
    const offset = (item.offset / length) * 100
    return {
      backgroundColor,
      color: textColor,
      width: `${widthPercent}%`,
      left: `calc(${offset}%) + ${marginLeft}px`,
      display: "absolute",
    } satisfies React.CSSProperties
  })

  const playheadPosition =
    typeof time === "number"
      ? `calc(${(time / length) * 100}% + ${marginLeft}px - (1.25rem / 2))`
      : undefined

  return (
    <section className="py-4 px-0.5">
      <div className="flex h-[36px] mt-2 gap-0.5 relative w-full bg-base-200">
        {typeof time === "number" && (
          <span
            style={{left: playheadPosition}}
            className="absolute bottom-[-10px] bg-gray-700 rounded-full w-5 h-5 z-10 border-2 border-gray-400"
          />
        )}
        {markPoints?.map((time, index) => (
          <span
            key={time}
            style={{left: `calc(${(time / length) * 100}% + ${marginLeft}px)`}}
            className="top-0 absolute py-2 bg-green-500 w-2 -translate-x-0.5 bg-opacity-50 h-[36px] z-10 cursor-pointer"
            onClick={() => onMarkerClick && onMarkerClick(time, index)}
          />
        ))}
        {items.map((item, index) => {
          const style = styles[index]
          return (
            <div
              key={index}
              className={clsx(
                "absolute text-sm cursor-pointer text-white py-2 text-center",
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
        })}
      </div>
      <TimeAxis length={length} />
    </section>
  )
}

export default Timeline
