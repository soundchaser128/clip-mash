import clsx from "clsx"
import {useCallback, useEffect, useState} from "react"
import {useForm} from "react-hook-form"
import ExternalLink from "../components/ExternalLink"
import {
  FolderType,
  Settings,
  cleanupFolder,
  getFileStats,
  getStashHealth,
  migratePreviewImages,
  setConfig,
} from "../api"
import {HiCheckCircle, HiCog, HiPhoto, HiTrash} from "react-icons/hi2"
import {useConfig} from "@/hooks/useConfig"
import Loader from "@/components/Loader"
import {formatBytes} from "@/helpers/formatting"
import {useCreateToast} from "@/hooks/useToast"
import {AspectRatio} from "@/components/VideoCard"
import useAspectRatioSetting from "@/hooks/useAspectRatioSetting"

type Inputs = Settings

interface HealthResult {
  success: boolean
  message: string
}

type FolderStats = [FolderType, number][]

const folderTypeNames: Record<FolderType, string> = {
  [FolderType.compilationVideo]: "Finished compilation videos",
  [FolderType.config]: "Configuration files",
  [FolderType.downloadedVideo]: "Downloaded videos",
  [FolderType.database]: "Database files",
  [FolderType.music]: "Music files",
  [FolderType.tempVideo]: "Temporary video files",
  [FolderType.previewImages]: "Preview images",
}

const canCleanup: FolderType[] = [
  FolderType.tempVideo,
  FolderType.compilationVideo,
]

const useFileStats = () => {
  const [stats, setStats] = useState<FolderStats>()
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error>()
  const [counter, setCounter] = useState(0)

  const refetch = useCallback(() => {
    setCounter((c) => c + 1)
  }, [])

  useEffect(() => {
    if (stats) {
      return
    }

    getFileStats()
      .then((stats) => setStats(stats as unknown as FolderStats))
      .catch((e) => setError(e as Error))
      .finally(() => setLoading(false))
  }, [stats, counter])

  return {stats, loading, error, refetch}
}

async function testCredentials(inputs: Inputs): Promise<HealthResult> {
  try {
    const response = await getStashHealth({
      apiKey: inputs.stash.apiKey,
      url: inputs.stash.stashUrl,
    })
    return {success: true, message: response}
  } catch (e) {
    const response = e as Response
    const text = await response.text()
    return {success: false, message: text}
  }
}

function AppConfigPage() {
  const config = useConfig()
  const {stats, loading, refetch} = useFileStats()
  const {watch, register, handleSubmit} = useForm<Inputs>({
    defaultValues: config,
  })
  const urlValue = watch("stash.stashUrl") || "http://localhost:9999"
  const apiKeyValue = watch("stash.apiKey")
  const stashSettingsPageUrl = `${urlValue}/settings?tab=security`
  const [healthResult, setHealthResult] = useState<HealthResult>()
  const createToast = useCreateToast()
  const [converting, setConverting] = useState(false)
  const [aspectRatio, setAspectRatio] = useAspectRatioSetting()

  const onSubmit = async (inputs: Inputs) => {
    inputs.stash.apiKey = inputs.stash.apiKey?.trim()
    inputs.stash.stashUrl = inputs.stash.stashUrl.trim()
    if (!inputs.handy?.enabled) {
      delete inputs.handy
    }

    await setConfig(inputs)
    // reload the window to re-fetch the config
    window.location.reload()
  }

  const onTestCredentials = useCallback(async () => {
    const response = await testCredentials({
      stash: {
        stashUrl: urlValue,
        apiKey: apiKeyValue,
      },
    })
    setHealthResult(response)
  }, [urlValue, apiKeyValue])

  const onCleanup = (type: FolderType) => async () => {
    if (!canCleanup.includes(type)) {
      return
    }

    try {
      await cleanupFolder(type)
      createToast({
        type: "success",
        message: "Finished cleaning up.",
      })
      refetch()
    } catch (e) {
      createToast({
        type: "error",
        message: "Error cleaning up: " + (e as Error).message,
      })
    }
  }

  const onConvertPreviewImages = async () => {
    setConverting(true)
    try {
      await migratePreviewImages()
    } finally {
      setConverting(false)
    }
  }

  return (
    <div className="flex flex-col pt-4 max-w-xl ml-auto mr-auto">
      <h1 className="text-3xl font-bold mb-4 text-center">Settings</h1>

      <section className="flex flex-col mb-4">
        <h2 className="text-xl font-bold mb-2">Interface</h2>
        <div className="form-control">
          <label className="label">
            <span className="label-text">Preview image aspect ratio</span>
          </label>

          <select
            value={aspectRatio}
            onChange={(e) => setAspectRatio(e.target.value as AspectRatio)}
            className="select select-sm select-bordered"
          >
            <option value="wide">Wide</option>
            <option value="square">Square</option>
            <option value="tall">Tall</option>
          </select>
        </div>
      </section>
      <form onSubmit={handleSubmit(onSubmit)} className="w-full">
        <section className="flex flex-col mb-4">
          <h2 className="text-xl font-bold mb-2">Stash configuration</h2>
          <div className="form-control">
            <label className="label" htmlFor="stashUrl">
              <span className="label-text">URL of your Stash instance:</span>
            </label>
            <input
              type="url"
              placeholder="Example: http://localhost:9999"
              className="input input-bordered"
              defaultValue="http://localhost:9999"
              {...register("stash.stashUrl")}
            />
          </div>

          <div className="form-control">
            <label className="label" htmlFor="apiKey">
              <span className="label-text">API key (optional):</span>
            </label>
            <input
              type="password"
              placeholder="eyJhbGc..."
              className="input input-bordered"
              {...register("stash.apiKey")}
            />
            <label className="label">
              <span className="label-text-alt">
                The API key is only required when authentication in Stash is
                enabled. Navigate to{" "}
                <ExternalLink href={stashSettingsPageUrl}>
                  {stashSettingsPageUrl}
                </ExternalLink>{" "}
                to retrieve your API key.
              </span>
            </label>
          </div>
          <div className="w-full flex justify-between">
            <button
              onClick={onTestCredentials}
              type="button"
              className="btn btn-sm btn-secondary"
            >
              <HiCog className="w-6 h-6" />
              Test credentials
            </button>
          </div>
          {healthResult && (
            <div
              className={clsx(
                "mt-4",
                healthResult.success && "text-green-600",
                !healthResult.success && "text-red-600",
              )}
            >
              {healthResult.success
                ? "Credentials work!"
                : "Error validating credentials: "}
              <br />
              {!healthResult.success && <code>{healthResult.message}</code>}
            </div>
          )}
        </section>

        <section className="flex flex-col mb-4">
          <h2 className="text-xl font-bold mb-2">Handy settings</h2>
          <div className="form-control">
            <label className="label">
              <span className="label-text">Enable Handy integration</span>

              <input
                type="checkbox"
                className="checkbox checkbox-primary"
                {...register("handy.enabled")}
              />
            </label>
          </div>

          <div className="form-control">
            <label className="label">
              <span className="label-text">Handy connection key</span>
            </label>
            <input
              type="password"
              placeholder="Connection key"
              className="input input-bordered"
              disabled={!watch("handy.enabled")}
              {...register("handy.key")}
            />
          </div>

          <button type="submit" className="btn btn-success self-end mt-4">
            <HiCheckCircle className="w-6 h-6" />
            Submit
          </button>
        </section>
      </form>

      <section className="flex flex-col">
        <h2 className="text-xl font-bold mb-2">File statistics</h2>
        {!loading && stats && (
          <div className="self-center">
            <table className="table">
              <thead>
                <tr>
                  <th>Folder type</th>
                  <th>Size</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {stats.map(([type, size]) => (
                  <tr key={type}>
                    <th>{folderTypeNames[type as FolderType]}</th>
                    <td className="text-right">{formatBytes(size)}</td>
                    <td>
                      {canCleanup.includes(type) && (
                        <button
                          onClick={onCleanup(type)}
                          className="btn btn-sm btn-error"
                        >
                          <HiTrash />
                          Clean up
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
        {loading && <Loader />}
      </section>

      <section className="mt-4 flex justify-between items-center">
        <p className="label-text">
          Convert preview images to the WebP format.
          <br />
          Reduces disk usage of the preview image folder.
        </p>
        <button
          disabled={converting}
          onClick={onConvertPreviewImages}
          className="btn btn-primary"
        >
          <HiPhoto /> Convert
        </button>
      </section>
    </div>
  )
}

export default AppConfigPage
