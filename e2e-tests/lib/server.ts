const baseUrl = "http://127.0.0.1:5174";

const serverTimeout = 15 * 1000;
const interval = 250;

async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function waitUntilHealthy() {
  let elapsed = 0;

  while (elapsed < serverTimeout) {
    const response = await fetch(`${baseUrl}/api/system/health`);
    if (response.ok) {
      return;
    } else {
      await sleep(interval);
      elapsed += interval;
    }
  }
}

export async function restartServer() {
  await fetch(`${baseUrl}/api/system/restart`, {
    method: "POST",
  });

  await waitUntilHealthy();
}
