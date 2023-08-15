import React from "react"
import {render} from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import Modal from "../Modal"
import {vi, describe, it, expect} from "vitest"

describe("Modal", () => {
  it("renders children when isOpen is true", () => {
    const screen = render(
      <Modal isOpen>
        <div>Test Content</div>
      </Modal>,
    )

    expect(screen.getByText("Test Content")).toBeInTheDocument()
  })

  it("does not render children when isOpen is false", () => {
    const screen = render(
      <Modal isOpen={false}>
        <div>Test Content</div>
      </Modal>,
    )

    expect(screen.queryByText("Test Content")).not.toBeInTheDocument()
  })

  it("calls onClose when close button is clicked", async () => {
    const handleClose = vi.fn()

    const screen = render(
      <Modal isOpen={true} onClose={handleClose}>
        <div>Test Content</div>
      </Modal>,
    )
    await userEvent.click(screen.getByRole("button"))

    expect(handleClose).toHaveBeenCalledTimes(1)
  })

  it("applies className to root element", () => {
    const screen = render(
      <Modal isOpen={true} className="test-class">
        <div>Test Content</div>
      </Modal>,
    )

    expect(screen.getByTestId("modal-content")).toHaveClass("test-class")
  })
})
