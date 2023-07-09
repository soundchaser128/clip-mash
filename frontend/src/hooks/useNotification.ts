import {useCallback, useEffect} from "react"
import useVisibility from "./useVisibility"

const options = {
  icon: "/android-chrome-192x192.png",
}

export default function useNotification() {
  const windowVisible = useVisibility()

  useEffect(() => {
    if (Notification.permission === "default") {
      Notification.requestPermission().then((permission) => {
        if (permission === "granted") {
          new Notification("Notifications enabled.", options)
        }
      })
    }
  }, [])

  const sendNotification = useCallback(
    (title: string, body?: string) => {
      if (windowVisible && Notification.permission === "granted") {
        new Notification(title, {
          ...options,
          body,
        })
      }
    },
    [windowVisible],
  )

  return sendNotification
}
