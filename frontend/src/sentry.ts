import * as sentry from "@sentry/react"

const localStorageVar = "sentryEnabled"

const Sentry = {
  get enabled(): boolean | null {
    const isDev = !import.meta.env.PROD
    if (isDev) {
      return false
    }

    const value = localStorage.getItem(localStorageVar)
    if (value === null) {
      return null
    } else {
      return value === "true"
    }
  },

  set enabled(enabled: boolean | null) {
    localStorage.setItem(localStorageVar, enabled ? "true" : "false")
  },

  setup() {
    const enabled = this.enabled === true

    if (!enabled) {
      return
    }

    sentry.init({
      dsn: import.meta.env.VITE_CLIP_MASH_FRONTEND_SENTRY_URI,
      integrations: [
        new sentry.BrowserTracing({
          tracePropagationTargets: ["localhost"],
        }),
        new sentry.Replay(),
        new sentry.BrowserProfilingIntegration(),
      ],
      tracesSampleRate: 1.0,
      replaysSessionSampleRate: 0.1,
      replaysOnErrorSampleRate: 1.0,
    })
  },
}

export default Sentry
