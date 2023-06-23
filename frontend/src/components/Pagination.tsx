import clsx from "clsx"
import React from "react"
import {HiChevronLeft, HiChevronRight} from "react-icons/hi2"
import {Link, To} from "react-router-dom"

interface PaginationProps {
  currentPage: number
  totalPages: number
  prevLink: To
  nextLink: To
}

const Pagination: React.FC<PaginationProps> = ({
  currentPage,
  totalPages,
  prevLink,
  nextLink,
}) => {
  const hasNextPage = currentPage < totalPages - 1
  const hasPreviousPage = currentPage > 0

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
          !hasPreviousPage && "btn-disabled"
        )}
      >
        <HiChevronLeft className="mr-2" />
        Previous
      </Link>
      <span>
        Page <strong>{currentPage + 1}</strong> of <strong>{totalPages}</strong>
      </span>
      <Link
        to={nextLink}
        className={clsx(
          "btn",
          hasNextPage && "btn-primary",
          !hasNextPage && "btn-disabled"
        )}
      >
        Next
        <HiChevronRight className="ml-2" />
      </Link>
    </div>
  )
}

export default Pagination
