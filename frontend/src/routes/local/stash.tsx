import React, {useState} from "react"

const AddStashVideoPage: React.FC = () => {
  const [query, setQuery] = useState("")

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
          />
        </div>
      </section>
    </>
  )
}

export default AddStashVideoPage
