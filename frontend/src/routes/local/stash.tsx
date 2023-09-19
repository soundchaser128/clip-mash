import React, {useEffect, useState} from "react"
import {useLoaderData, useSearchParams} from "react-router-dom"
import {ListVideoDtoPage} from "../../api"
import VideoCard from "../../components/VideoCard"

const AddStashVideoPage: React.FC = () => {
  const [search, setSearchParams] = useSearchParams({query: ""})
  const [query, setQuery] = useState(search.get("query") || "")

  const data = useLoaderData() as ListVideoDtoPage
  console.log(data)

  useEffect(() => {
    setSearchParams({query})
  }, [query])

  return (
    <>
      <h1 className="text-3xl text-center font-bold mb-4">
        Add videos from Stash
      </h1>
      <section className="flex max-w-xl self-center">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Search for videos</span>
          </label>
          <input
            required
            type="text"
            className="input input-bordered w-96"
            placeholder="Enter query..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>
      </section>
      <section>
        {data.content.map((video) => (
          <VideoCard key={video.video.id.id} video={video} />
        ))}
      </section>
    </>
  )
}

export default AddStashVideoPage
