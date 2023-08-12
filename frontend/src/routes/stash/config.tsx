import clsx from "clsx"
import {useCallback, useState} from "react"
import {useForm} from "react-hook-form"
import {useLoaderData, useNavigate} from "react-router-dom"
import ExternalLink from "../../components/ExternalLink"

interface Inputs {
  stashUrl: string
  apiKey: string
}

interface HealthResult {
  success: boolean
  message: string
}

async function testCredentials(inputs: Inputs): Promise<HealthResult> {
  const params = new URLSearchParams()
  params.set("apiKey", inputs.apiKey)
  params.set("url", inputs.stashUrl)
  const url = `/api/stash/health?${params.toString()}`
  const response = await fetch(url)
  if (response.ok) {
    const status = await response.json()
    return {success: true, message: status}
  } else {
    const text = await response.json()
    return {success: false, message: text.error}
  }
}

function ConfigPage() {
  const config = useLoaderData() as Inputs
  const {watch, register, handleSubmit} = useForm<Inputs>({
    defaultValues: config,
  })
  const urlValue = watch("stashUrl") || "http://localhost:9999"
  const apiKeyValue = watch("apiKey")
  const settingsPage = `${urlValue}/settings?tab=security`
  const navigate = useNavigate()
  const [healthResult, setHealthResult] = useState<HealthResult>()

  const onSubmit = async (inputs: Inputs) => {
    const health = await testCredentials(inputs)
    inputs.apiKey = inputs.apiKey.trim()
    inputs.stashUrl = inputs.stashUrl.trim()

    if (health.success) {
      const response = await fetch("/api/stash/config", {
        method: "POST",
        body: JSON.stringify(inputs),
        headers: {"content-type": "application/json"},
      })
      if (response.ok) {
        navigate("/")
      }
    } else {
      // TODO
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
    <section className="py-4 flex flex-col">
      <h1 className="text-5xl text-brand font-bold mb-4 text-center">
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
            Test credentials
          </button>
          <button type="submit" className="btn btn-success">
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

export default ConfigPage
