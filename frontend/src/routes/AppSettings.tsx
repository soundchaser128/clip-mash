import clsx from "clsx"
import {useCallback, useEffect, useState} from "react"
import {useForm} from "react-hook-form"
import ExternalLink from "../components/ExternalLink"
import {FolderType, getFileStats, getHealth, setConfig} from "../api"
import {HiCheckCircle, HiCog, HiTrash} from "react-icons/hi2"
import {useConfig} from "@/hooks/useConfig"
import Loader from "@/components/Loader"
import {formatBytes} from "@/helpers"

interface Inputs {
  stashUrl: string
  apiKey: string
}

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
}

const canCleanup: FolderType[] = [
  FolderType.tempVideo,
  FolderType.compilationVideo,
]

const useFileStats = () => {
  const [stats, setStats] = useState<FolderStats>()
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error>()

  useEffect(() => {
    if (stats) {
      return
    }

    getFileStats()
      .then((stats) => setStats(stats as unknown as FolderStats))
      .catch((e) => setError(e as Error))
      .finally(() => setLoading(false))
  }, [stats])

  return {stats, loading, error}
}

async function testCredentials(inputs: Inputs): Promise<HealthResult> {
  try {
    const response = await getHealth({
      apiKey: inputs.apiKey,
      url: inputs.stashUrl,
    })
    return {success: true, message: response}
  } catch (e) {
    const response = e as Response
    const text = await response.text()
    return {success: false, message: text}
  }
}

function StashConfigPage() {
  const config = useConfig()
  const {stats, loading} = useFileStats()
  const {watch, register, handleSubmit} = useForm<Inputs>({
    defaultValues: config,
  })
  const urlValue = watch("stashUrl") || "http://localhost:9999"
  const apiKeyValue = watch("apiKey")
  const settingsPage = `${urlValue}/settings?tab=security`
  const [healthResult, setHealthResult] = useState<HealthResult>()

  const onSubmit = async (inputs: Inputs) => {
    const health = await testCredentials(inputs)
    inputs.apiKey = inputs.apiKey.trim()
    inputs.stashUrl = inputs.stashUrl.trim()

    if (health.success) {
      await setConfig(inputs)
      window.location.href = "/library/add/stash"
    }
  }

  const onTestCredentials = useCallback(async () => {
    const response = await testCredentials({
      stashUrl: urlValue,
      apiKey: apiKeyValue,
    })
    setHealthResult(response)
  }, [urlValue, apiKeyValue])

  return (
    <div className="flex flex-col pt-4">
      <h1 className="text-3xl font-bold mb-4 text-center">Settings</h1>

      <section className="flex flex-col">
        <h2 className="text-xl font-bold mb-2 text-center">File statistics</h2>
        {!loading && stats && (
          <div className="max-w-lg self-center">
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
                        <button className="btn btn-sm btn-error">
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

      <section className="flex flex-col min-h-screen">
        <h2 className="text-xl font-bold mb-2 text-center">
          Stash configuration
        </h2>
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="max-w-lg w-full self-center"
        >
          <div className="form-control">
            <label className="label">
              <span className="label-text">URL of your Stash instance:</span>
            </label>
            <input
              type="url"
              placeholder="Example: http://localhost:9999"
              className="input input-bordered"
              defaultValue="http://localhost:9999"
              required
              {...register("stashUrl", {required: true})}
            />
          </div>

          <div className="form-control">
            <label className="label">
              <span className="label-text">API key:</span>
            </label>
            <input
              type="text"
              placeholder="eyJhbGc..."
              className="input input-bordered"
              required
              {...register("apiKey", {required: true})}
            />
            <label className="label">
              <span className="label-text-alt">
                Navigate to{" "}
                <ExternalLink href={settingsPage}>{settingsPage}</ExternalLink>{" "}
                to retrieve your API key.
              </span>
            </label>
          </div>
          <div className="w-full flex justify-between">
            <button
              onClick={onTestCredentials}
              type="button"
              className="btn btn-secondary"
            >
              <HiCog className="w-6 h-6" />
              Test credentials
            </button>
            <button type="submit" className="btn btn-success">
              <HiCheckCircle className="w-6 h-6" />
              Submit
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
        </form>
      </section>
    </div>
  )
}

export default StashConfigPage
