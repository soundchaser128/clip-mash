import {useEffect} from "react"
import {HiMoon, HiSun} from "react-icons/hi2"
import useLocalStorage from "../hooks/useLocalStorage"

type Theme = "clip-mash-dark" | "clip-mash-light"

const ThemeSwitch = () => {
  const [theme, setTheme] = useLocalStorage<Theme>("theme", "clip-mash-light")

  const toggleTheme = () => {
    if (theme === "clip-mash-dark") {
      setTheme("clip-mash-light")
    } else {
      setTheme("clip-mash-dark")
    }
  }

  useEffect(() => {
    document.body.dataset["theme"] = theme
  }, [theme])

  return (
    <button
      className="btn shadow-lg btn-circle absolute bottom-6 right-6"
      onClick={toggleTheme}
    >
      {theme === "clip-mash-dark" ? (
        <HiSun className="w-6 h-6" />
      ) : (
        <HiMoon className="w-6 h-6" />
      )}
    </button>
  )
}

export default ThemeSwitch
