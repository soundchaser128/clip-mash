import {useForm} from "react-hook-form"
import Field from "../../../components/Field"
import {HiArrowDownTray, HiPlus, HiXMark} from "react-icons/hi2"
import {useNavigate} from "react-router"
import useNotification from "../../../hooks/useNotification"
import Loader from "../../../components/Loader"
import {useState} from "react"
import {pluralize} from "@/helpers/formatting"
import {AddVideosRequest, addNewVideos} from "../../../api"
import ExternalLink from "@/components/ExternalLink"
import AddTagModal from "@/components/AddTagModal"

const splitRegex = /\s+|\n/

const validateUrl = (url: string): string | undefined => {
  try {
    new URL(url)
    return undefined
  } catch (err: unknown) {
    return (err as Error).message
  }
}

const DownloadVideosPage: React.FC = () => {
  const navigate = useNavigate()
  const sendNotification = useNotification()
  const [videoCount, setVideoCount] = useState<number>()
  const [urls, setUrls] = useState("")
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errors, setErrors] = useState<string[]>([])
  const [tags, setTags] = useState<string[]>([])
  const [editingTags, setEditingTags] = useState(false)

  const onSubmit = async () => {
    setIsSubmitting(true)

    const urlList = urls
      .split(splitRegex)
      .map((res) => res.trim())
      .filter((u) => u.length > 0)
    setVideoCount(urlList.length)
    const errors = []
    for (const url of urlList) {
      const validationError = validateUrl(url)
      if (validationError) {
        errors.push(`Invalid URL ${url}: ${validationError}`)
      }
    }
    if (errors.length > 0) {
      setErrors(errors)
      setIsSubmitting(false)
    } else {
      const body: AddVideosRequest = {
        type: "download",
        urls: urlList,
        tags,
      }
      await addNewVideos(body)
    }

    if (errors.length === 0) {
      sendNotification("Success", "Video downloads finished!")
      navigate("/library")
    }
  }

  const onShowModal = () => {
    setEditingTags(true)
  }

  const onAddTag = (tag: string) => {
    setTags((prev) => [...prev, tag])
    setEditingTags(false)
  }

  if (isSubmitting) {
    return (
      <Loader className="self-center">
        Downloading {videoCount} {pluralize("video", videoCount)}...
      </Loader>
    )
  }

  return (
    <>
      {errors.length > 0 && (
        <div className="bg-error p-4 shadow-xl rounded-xl text-error-content max-w-2xl self-center mb-4 relative">
          <HiXMark
            className="absolute top-2 right-2 w-6 h-6 cursor-pointer"
            onClick={() => setErrors([])}
          />
          <p className="font-bold">Some downloads failed:</p>
          {errors.map((error, i) => (
            <p key={i} className="text-sm">
              {error}
            </p>
          ))}
        </div>
      )}
      <form
        onSubmit={onSubmit}
        className="max-w-lg w-full self-center flex flex-col"
      >
        <p className="mb-4 text-gray-500">
          You can download videos from YouTube, Vimeo or any other site that{" "}
          <ExternalLink href="https://github.com/yt-dlp/yt-dlp">
            yt-dlp
          </ExternalLink>{" "}
          supports.
        </p>
        <Field name="urls" label="Video URLs">
          <textarea
            className="textarea textarea-bordered"
            placeholder="Enter URLs separated by line breaks..."
            rows={5}
            value={urls}
            onChange={(e) => setUrls(e.target.value)}
          />
        </Field>

        <div className="flex justify-between mt-4">
          <ul className="inline-flex flex-wrap gap-y-1 gap-x-0.5 -ml-2">
            {tags.map((tag) => (
              <li key={tag} className="badge">
                {tag}
              </li>
            ))}
          </ul>
          <button
            onClick={onShowModal}
            type="button"
            className="btn btn-sm self-end"
          >
            <HiPlus /> Add tag(s) to all videos
          </button>
        </div>

        <button
          type="submit"
          disabled={isSubmitting}
          className="mt-4 btn btn-success self-end"
        >
          <HiArrowDownTray className="mr-2" />
          Download
        </button>
      </form>

      <AddTagModal
        isOpen={editingTags}
        onSubmit={onAddTag}
        onClose={() => setEditingTags(false)}
      />
    </>
  )
}

export default DownloadVideosPage
