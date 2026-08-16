export function saveJsonToDisk<T>(fileName: string, data: T) {
  const json = JSON.stringify(data)
  const blob = new Blob([json], {type: "application/json"})
  saveBlobToDisk(fileName, blob)
}

export function saveBlobToDisk(fileName: string, blob: Blob) {
  const href = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = href
  link.download = fileName
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(href)
}
