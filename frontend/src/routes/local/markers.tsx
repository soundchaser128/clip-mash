import {useLoaderData} from "react-router-dom"
import {MarkerDto} from "../../types.generated"
import MarkerPage from "../../components/MarkerPage"
import {LocalFilesFormStage, Page} from "../../types/types"

export default function LocalMarkersPage() {
  const data = useLoaderData() as Page<MarkerDto>

  return (
    <MarkerPage
      data={{markers: data.content}}
      withImages
      nextStage={LocalFilesFormStage.Music}
    />
  )
}
