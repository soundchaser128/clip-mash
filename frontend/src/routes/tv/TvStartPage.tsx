import React, {useState} from "react"
import {HiRocketLaunch} from "react-icons/hi2"
import {useNavigate} from "react-router-dom"

const TvStartPage: React.FC = () => {
  const [query, setQuery] = useState<string>("")
  const navigate = useNavigate()

  const onSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault()

    navigate(`/tv/watch/${query}`)
  }

  return (
    <div className="flex justify-center items-center h-screen">
      <form onSubmit={onSubmit} className="flex gap-2 items-center">
        <input
          className="input input-primary input-lg"
          type="text"
          placeholder="What do you want to watch?"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        <button className="btn btn-primary btn-lg ml-2">
          <HiRocketLaunch />
          Go
        </button>
      </form>
    </div>
  )
}

export default TvStartPage
