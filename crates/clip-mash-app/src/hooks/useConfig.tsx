import {createContext, useContext, useState, useEffect} from "react"
import {Settings, getConfig} from "../api"

const ConfigContext = createContext<Settings | undefined>(undefined)

export const useConfig = () => {
  const config = useContext(ConfigContext)
  return config
}

interface ConfigProviderProps {
  children: React.ReactNode
}

export const ConfigProvider: React.FC<{children: React.ReactNode}> = ({
  children,
}: ConfigProviderProps) => {
  const [currentConfig, setCurrentConfig] = useState<Settings>()

  useEffect(() => {
    if (!currentConfig) {
      getConfig()
        .then((config) => setCurrentConfig(config))
        .catch((e) => console.error(e))
    }
  }, [])

  return (
    <ConfigContext.Provider value={currentConfig}>
      {children}
    </ConfigContext.Provider>
  )
}
