import {HiCheck, HiPlus} from "react-icons/hi2"
import {useNavigate, useSearchParams} from "react-router-dom"
import {useForm} from "react-hook-form"
import {useEffect, useState} from "react"
import Loader from "@/components/Loader"
import {addNewVideos, listFileEntries, ListFileEntriesResponse} from "@/api"
import FileBrowser from "@/components/FileBrowser"
import {useCreateToast} from "@/hooks/useToast"
import AddTagModal from "@/components/AddTagModal"

interface Inputs {
  path: string
  recurse: boolean
}

export default function SelectVideos() {
  const navigate = useNavigate()
  const [files, setFiles] = useState<ListFileEntriesResponse>()
  const [tags, setTags] = useState<string[]>([])
  const [editingTags, setEditingTags] = useState(false)
  const [query, setQuery] = useSearchParams()
  const path = query.get("path")
  const [submitting, setSubmitting] = useState(false)
  const {register, handleSubmit, control, setValue} = useForm<Inputs>({
    defaultValues: {
      path: path || "",
    },
  })

  const createToast = useCreateToast()

  const onSubmit = async (values: Inputs) => {
    setSubmitting(true)
    await addNewVideos({
      type: "local",
      path: values.path,
      recurse: values.recurse,
      tags,
    })
    navigate("/library")
  }

  const fetchEntries = async (path?: string) => {
    const entries = await listFileEntries({path})
    setValue("path", entries.directory)
    return entries
  }

  useEffect(() => {
    fetchEntries(path || undefined)
      .then((entries) => setFiles(entries))
      .catch(async (error: unknown) => {
        let message
        if (error instanceof Error) {
          message = error.message
        } else if (error instanceof Response) {
          const object = (await error.json()) as {error: string}
          const payload = JSON.parse(object.error) as {error: string}
          message = payload.error
        }
        createToast({
          message: `Could not access path '${path}': ${message}`,
          type: "error",
        })
      })
  }, [path])

  const onSelectEntry = (path: string) => {
    setValue("path", path)
    setQuery({path})
  }

  const onShowModal = () => {
    setEditingTags(true)
  }

  const onAddTag = (tag: string) => {
    setTags((prev) => [...prev, tag])
    setEditingTags(false)
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
              name="path"
              drives={files?.drives || []}
              files={files?.entries || []}
              onSelectItem={(e) => onSelectEntry(e.fullPath)}
              control={control}
            />
            <div className="flex justify-between mt-2 w-full items-center">
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

      <AddTagModal
        isOpen={editingTags}
        onSubmit={onAddTag}
        onClose={() => setEditingTags(false)}
      />
    </>
  )
}
