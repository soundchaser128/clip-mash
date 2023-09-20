import {createContext, useContext, useState, useEffect} from "react"
import {Config, getConfig} from "../api"

const ConfigContext = createContext<Config | undefined>(undefined)

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
  const [currentConfig, setCurrentConfig] = useState<Config>()

  useEffect(() => {
    getConfig().then((config) => setCurrentConfig(config))
  }, [])

  return (
    <ConfigContext.Provider value={currentConfig}>
      {children}
    </ConfigContext.Provider>
  )
}
