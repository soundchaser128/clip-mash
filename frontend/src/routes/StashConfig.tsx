import clsx from "clsx"
import {useCallback, useState} from "react"
import {useForm} from "react-hook-form"
import {useLoaderData} from "react-router-dom"
import ExternalLink from "../components/ExternalLink"
import {getHealth, setConfig} from "../api"
import {HiCheckCircle, HiCog} from "react-icons/hi2"

interface Inputs {
  stashUrl: string
  apiKey: string
}

interface HealthResult {
  success: boolean
  message: string
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
  const config = useLoaderData() as Inputs
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
    <section className="py-4 flex flex-col min-h-screen">
      <h1 className="text-3xl text-primary font-bold mb-4 text-center">
        Stash configuration setup
      </h1>
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
              <ExternalLink href={settingsPage}>{settingsPage}</ExternalLink> to
              retrieve your API key.
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
  )
}

export default StashConfigPage
