import {
  HiArchiveBox,
  HiArrowDownTray,
  HiBookOpen,
  HiFolder,
} from "react-icons/hi2"
import {Link} from "react-router-dom"

const AddVideosPage = () => {
  return (
    <>
      <h1 className="text-3xl font-bold text-center mb-4">Add Videos</h1>
      <div className="max-w-xl self-center">
        <div className="mb-6">
          You can add videos from different sources:
          <ul className="list-disc list-inside">
            <li>
              Download from any site <code>yt-dlp</code> supports.
            </li>
            <li>Scan a local folder for video files.</li>
            <li>Connect to your Stash instance to add files from there.</li>
          </ul>
        </div>
      </div>
      <div className="self-center flex gap-1">
        <Link
          to="/library/add/download"
          className="btn btn-primary btn-lg w-48"
        >
          <HiArrowDownTray />
          Download
        </Link>

        <Link to="/library/add/folder" className="btn btn-primary btn-lg w-48">
          <HiFolder />
          Folder
        </Link>

        <Link to="/library/add/stash" className="btn btn-primary btn-lg w-48">
          <HiArchiveBox />
          Stash
        </Link>

        <Link
          to="/library/add/alexandria"
          className="btn btn-primary btn-lg w-48"
        >
          <HiBookOpen />
          Alexandria
        </Link>
      </div>
    </>
  )
}

export default AddVideosPage
