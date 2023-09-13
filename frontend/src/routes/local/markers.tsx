import {useLoaderData} from "react-router-dom"
import {MarkerDto} from "../../api"
import MarkerPage from "../../components/MarkerPage"
import {FormStage} from "../../types/form-state"

export default function LocalMarkersPage() {
  const data = useLoaderData() as MarkerDto[]

  return (
    <MarkerPage data={{markers: data}} nextStage={FormStage.Music} />
  )
}
