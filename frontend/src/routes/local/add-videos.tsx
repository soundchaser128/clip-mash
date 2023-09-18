import {HiArchiveBox, HiArrowDownTray, HiFolder} from "react-icons/hi2"
import {Link} from "react-router-dom"

const AddVideosPage = () => {
  return (
    <>
      <h1 className="text-3xl font-bold text-center mb-4">Add Videos</h1>
      <div className="max-w-xl self-center">
        <p className="mb-6">
          You can add videos from different sources:
          <ul className="list-disc list-inside">
            <li>
              Download from any site <code>yt-dlp</code> supports.
            </li>
            <li>Scan a local folder for video files</li>
            <li>Connect to your Stash instance to add files from there.</li>
          </ul>
        </p>
      </div>
      <div className="self-center flex gap-1">
        <Link to="/library/add/download" className="btn btn-primary btn-lg">
          <HiArrowDownTray />
          Download
        </Link>

        <Link to="/library/add/folder" className="btn btn-primary btn-lg">
          <HiFolder />
          Folder
        </Link>

        <Link to="/library/add/stash" className="btn btn-primary btn-lg">
          <HiArchiveBox />
          Stash
        </Link>
      </div>
    </>
  )
}

export default AddVideosPage
