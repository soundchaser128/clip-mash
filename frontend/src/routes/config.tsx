import clsx from "clsx"
import {useState} from "react"
import {useForm} from "react-hook-form"
import {useNavigate} from "react-router-dom"

interface Inputs {
  stashUrl: string
  apiKey: string
}

async function testCredentials(inputs: Inputs) {
  const params = new URLSearchParams()
  params.set("apiKey", inputs.apiKey)
  params.set("url", inputs.stashUrl)
  const url = `/api/health?${params.toString()}`
  const response = await fetch(url)
  if (response.ok) {
    const status = await response.json()
    return status
  } else {
    const text = await response.text()
    return text
  }
}

function ConfigPage() {
  const {watch, register, handleSubmit} = useForm<Inputs>()
  const urlValue = watch("stashUrl") || "http://localhost:9999"
  const apiKeyValue = watch("apiKey")
  const settingsPage = `${urlValue}/settings?tab=security`
  const navigate = useNavigate()
  const [healthResult, setHealthResult] = useState<string>()

  const onSubmit = async (inputs: Inputs) => {
    const health = await testCredentials(inputs)
    if (health === "OK") {
      const response = await fetch("/api/config", {
        method: "POST",
        body: JSON.stringify(inputs),
        headers: {"content-type": "application/json"},
      })
      if (response.ok) {
        navigate("/")
      }
    } else {
      // todo
    }
  }

  const onTestCredentials = async () => {
    const response = await testCredentials({
      stashUrl: urlValue,
      apiKey: apiKeyValue,
    })
    setHealthResult(response)
  }

  return (
    <main className="container ml-auto mr-auto">
      <section className="py-4 flex flex-col">
        <h1 className="text-4xl font-bold mb-4 text-center">
          Configuration setup
        </h1>
        <form onSubmit={handleSubmit(onSubmit)} className="max-w-lg w-full">
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
                <a className="link" href={settingsPage} target="_blank">
                  {settingsPage}
                </a>{" "}
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
                healthResult === "OK" && "text-green-400",
                healthResult !== "OK" && "text-red-600"
              )}
            >
              {healthResult === "OK"
                ? "Credentials work: "
                : "Error validating credentials: "}{" "}
              <pre>{healthResult}</pre>
            </div>
          )}
        </form>
      </section>
    </main>
  )
}

export default ConfigPage
