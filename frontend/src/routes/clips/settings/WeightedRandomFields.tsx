import {useLoaderData} from "react-router-dom"
import WeightsModal from "./WeightsModal"
import {ClipsLoaderData} from "@/routes/loaders"

const WeightedRandomFields: React.FC = () => {
  const data = useLoaderData() as ClipsLoaderData

  return (
    <>
      <WeightsModal className="mt-4" clips={data.clips} />
    </>
  )
}

export default WeightedRandomFields
