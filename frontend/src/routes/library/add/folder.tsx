import {HiCheck} from "react-icons/hi2"
import {useNavigate, useSearchParams} from "react-router-dom"
import {useForm} from "react-hook-form"
import {useEffect, useState} from "react"
import Loader from "../../../components/Loader"
import {
  addNewVideos,
  listFileEntries,
  ListFileEntriesResponse,
  FileSystemEntry,
} from "../../../api"
import FileBrowser from "@/components/FileBrowser"

interface Inputs {
  path: string
  recurse: boolean
  fileName: string
}

export default function SelectVideos() {
  const navigate = useNavigate()
  const [submitting, setSubmitting] = useState(false)
  const {register, handleSubmit} = useForm<Inputs>({})
  const [files, setFiles] = useState<ListFileEntriesResponse>()
  const [query, setQuery] = useSearchParams()
  const path = query.get("path")

  const onSubmit = async (values: Inputs) => {
    setSubmitting(true)
    await addNewVideos({
      type: "local",
      path: values.path,
      recurse: values.recurse,
    })
    navigate("/library")
  }

  const fetchEntries = async (path?: string) => {
    const entries = await listFileEntries({path})
    return entries
  }

  useEffect(() => {
    fetchEntries(path || undefined).then((entries) => setFiles(entries))
  }, [path])

  const onSelectEntry = (path: string) => {
    setQuery({path})
  }

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex gap-4 items-start flex-col self-center w-[36rem] grow"
      >
        {!submitting && (
          <>
            <FileBrowser
              currentPath={files?.directory || ""}
              files={files?.entries || []}
              onSelectItem={(e) => onSelectEntry(e.fullPath)}
              onPathChange={onSelectEntry}
            />
            <div className="form-control justify-between w-full">
              <label className="label cursor-pointer">
                <span className="label-text">
                  Look at all the subdirectories as well
                </span>
                <input
                  type="checkbox"
                  className="toggle"
                  {...register("recurse")}
                />
              </label>
            </div>
            <button type="submit" className="btn btn-success self-end">
              <HiCheck className="mr-2" />
              Submit
            </button>
          </>
        )}

        {submitting && (
          <Loader>
            Scanning your videos and generating preview images...
            <br /> This might take a while, depending on how many videos you
            have.
          </Loader>
        )}
      </form>
    </>
  )
}
