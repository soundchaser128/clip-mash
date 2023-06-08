import React from "react"
import { render, screen, fireEvent } from "@testing-library/react"
import Modal from "../Modal"

describe("Modal", () => {
  it("renders children when isOpen is true", () => {
    render(
      <Modal isOpen={true}>
        <div>Test Content</div>
      </Modal>
    )

    expect(screen.getByText("Test Content")).toBeInTheDocument()
  })

  it("does not render children when isOpen is false", () => {
    render(
      <Modal isOpen={false}>
        <div>Test Content</div>
      </Modal>
    )

    expect(screen.queryByText("Test Content")).not.toBeInTheDocument()
  })

  it("calls onClose when close button is clicked", () => {
    const handleClose = jest.fn()

    render(
      <Modal isOpen={true} onClose={handleClose}>
        <div>Test Content</div>
      </Modal>
    )

    fireEvent.click(screen.getByRole("button"))

    expect(handleClose).toHaveBeenCalledTimes(1)
  })

  it("applies className to root element", () => {
    render(
      <Modal isOpen={true} className="test-class">
        <div>Test Content</div>
      </Modal>
    )

    expect(screen.getByTestId("modal-root")).toHaveClass("test-class")
  })

  it("applies size and position props correctly", () => {
    render(
      <Modal isOpen={true} size="fluid" position="top">
        <div>Test Content</div>
      </Modal>
    )

    expect(screen.getByTestId("modal-content")).toHaveClass("top-4")
    expect(screen.getByTestId("modal-content")).not.toHaveClass("w-[95vw]")
  })
})