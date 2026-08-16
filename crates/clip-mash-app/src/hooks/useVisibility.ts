import {useEffect, useState} from "react"

export default function useVisibility() {
  const [visible, setVisible] = useState<boolean>()
  useEffect(() => {
    document.addEventListener("visibilitychange", () => {
      if (document.visibilityState === "visible") {
        setVisible(true)
      } else {
        setVisible(false)
      }
    })
  }, [])

  return visible
}
