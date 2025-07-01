// format number of bytes as human readable string
export function formatBytes(bytes: number): string {
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"]
  if (bytes === 0) {
    return "0 Bytes"
  }
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${parseFloat((bytes / Math.pow(1024, i)).toFixed(2))} ${sizes[i]}`
}

export function pluralize(
  word: string,
  count: number | undefined | null,
): string {
  return count === 1 ? word : `${word}s`
}
