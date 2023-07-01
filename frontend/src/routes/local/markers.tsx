import {useLoaderData} from "react-router-dom"
import {MarkerDto} from "../../types.generated"
import MarkerPage from "../../components/MarkerPage"
import {LocalFilesFormStage} from "../../types/types"

export default function LocalMarkersPage() {
  const data = useLoaderData() as MarkerDto[]

  return (
    <MarkerPage data={{markers: data}} nextStage={LocalFilesFormStage.Music} />
  )
}
