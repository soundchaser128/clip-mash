import clsx from "clsx"
import Footer from "./Footer"
import {useRouteLoaderData} from "react-router-dom"
import ThemeSwitch from "./ThemeSwitch"

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

  return (
    <div className={clsx(styles.root, isLoading && "opacity-25")}>
      <main className={styles.main}>{children}</main>
      <Footer version={version} />
      <ThemeSwitch />
    </div>
  )
}

export default Layout
