import clsx from "clsx"
import React from "react"
import {HiChevronLeft, HiChevronRight} from "react-icons/hi2"
import {Link, To, useSearchParams} from "react-router-dom"

interface PaginationProps {
  currentPage: number
  totalPages: number
  startIndex?: 0 | 1
}

function setParam(
  searchParams: URLSearchParams,
  name: string,
  value: string,
): To {
  const params = new URLSearchParams(searchParams)
  params.set(name, value.toString())
  return {search: `?${params.toString()}`}
}

const Pagination: React.FC<PaginationProps> = ({
  currentPage,
  totalPages,
  startIndex = 0,
}) => {
  const hasNextPage =
    startIndex === 1 ? currentPage < totalPages : currentPage < totalPages - 1
  const hasPreviousPage = currentPage > startIndex
  const [searchParams] = useSearchParams()
  const nextLink: To = setParam(
    searchParams,
    "page",
    (currentPage + 1).toString(),
  )
  const prevLink: To = setParam(
    searchParams,
    "page",
    (currentPage - 1).toString(),
  )

  console.log({currentPage, nextLink, prevLink})

  if (totalPages <= 1) {
    return null
  }

  return (
    <div className="w-full flex justify-between items-center">
      <Link
        to={prevLink}
        className={clsx(
          "btn",
          hasPreviousPage && "btn-primary",
          !hasPreviousPage && "btn-disabled",
        )}
      >
        <HiChevronLeft className="mr-2" />
        Previous
      </Link>
      <span>
        Page <strong>{startIndex === 0 ? currentPage + 1 : currentPage}</strong>{" "}
        of <strong>{totalPages}</strong>
      </span>
      <Link
        to={nextLink}
        className={clsx(
          "btn",
          hasNextPage && "btn-primary",
          !hasNextPage && "btn-disabled",
        )}
      >
        Next
        <HiChevronRight className="ml-2" />
      </Link>
    </div>
  )
}

export default Pagination
