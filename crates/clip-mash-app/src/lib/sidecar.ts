import {getCurrentWindow} from "@tauri-apps/api/window"
import {Child, Command} from "@tauri-apps/plugin-shell"

const BACKEND = "../../../target/release/clip-mash-server"

class Sidecar {
  process: Child | null = null

  async start() {
    if (this.process) {
      console.log("Sidecar process is already running.")
      return
    }
    const command = Command.sidecar(BACKEND)
    this.process = await command.spawn()
    console.log("Sidecar process started.")

    getCurrentWindow().onCloseRequested(async () => {
      if (this.process) {
        await this.process.kill()
        console.log("Sidecar process killed.")
      }
      this.process = null
    })
  }
}

export const sidecar = new Sidecar()
