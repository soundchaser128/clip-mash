import {Outlet} from "react-router-dom"
import Layout from "../../components/Layout"

const LocalFilesRoot: React.FC = () => {
  return (
    <Layout>
      <h1 className="text-5xl font-bold my-4 text-center">ClipMash</h1>
      <Outlet />
    </Layout>
  )
}

export default LocalFilesRoot
