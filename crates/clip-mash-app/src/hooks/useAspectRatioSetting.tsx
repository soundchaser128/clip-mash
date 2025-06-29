import {AspectRatio} from "@/components/VideoCard"
import useLocalStorage from "./useLocalStorage"

function useAspectRatioSetting(): [AspectRatio, (value: AspectRatio) => void] {
  const [aspectRatio, setAspectRatio] = useLocalStorage<AspectRatio>(
    "videoGridAspectRatio",
    "wide",
  )

  return [aspectRatio, setAspectRatio]
}

export default useAspectRatioSetting
