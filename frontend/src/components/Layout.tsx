import clsx from "clsx"
import Footer from "./Footer"
import {useRouteLoaderData} from "react-router-dom"
import ThemeSwitcher from "./ThemeSwitcher"
import {useToast} from "@/hooks/useToast"
import Toast from "./Toast"
import SentryInfo from "./SentryInfo"

interface Props {
  children: React.ReactNode
  isLoading?: boolean
}

const styles = {
  root: "min-h-screen flex flex-col justify-between transition",
  main: "flex flex-col container ml-auto mr-auto px-1",
}

const Layout: React.FC<Props> = ({children, isLoading}) => {
  const version = useRouteLoaderData("root") as string
  const toast = useToast()

  return (
    <div className={clsx(styles.root, isLoading && "opacity-25")}>
      <SentryInfo />
      {toast?.data && (
        <Toast type={toast.data.type}>{toast.data.message}</Toast>
      )}
      <main className={styles.main}>{children}</main>
      <Footer version={version} />
      <ThemeSwitcher />
    </div>
  )
}

export default Layout
