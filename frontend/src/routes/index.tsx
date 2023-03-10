import clsx from "clsx";
import { useEffect, useState } from "react";
import { useForm, SubmitHandler, useFieldArray } from "react-hook-form";
import { Form, useLoaderData } from "react-router-dom";

type Inputs = {
  mode: "none" | "tags" | "performers";
  filter: string;
};

interface Tag {
  name: string;
  id: string;
  count: number;
}

interface Performer {
  name: string;
  id: string;
  sceneCount: number;
  imageUrl?: string;
}

interface Data {
  performers: Performer[];
  tags: Tag[];
}

// e FormStage = "select-mode" | "select-criteria" | "video-options" | "wait"
enum FormStage {
  SelectMode = 1,
  SelectCriteria = 2,
  VideoOptions = 3,
  Wait = 4,
}

async function fetchTags(): Promise<Tag[]> {
  const response = await fetch("/api/tags");
  return await response.json();
}

async function fetchPerformers(): Promise<Performer[]> {
  const response = await fetch("/api/performers");
  return await response.json();
}

export async function loader(): Promise<Data> {
  const [tags, performers] = await Promise.all([
    fetchTags(),
    fetchPerformers(),
  ]);

  return { tags, performers };
}

function filterData(data: Data, filter?: string): Data {
  if (!filter || filter.trim().length === 0) {
    return data;
  } else {
    return {
      performers: data.performers.filter((p) =>
        p.name.toLowerCase().includes(filter.toLowerCase())
      ),
      tags: data.tags.filter((t) =>
        t.name.toLowerCase().includes(filter.toLowerCase())
      ),
    };
  }
}

function App() {
  const { register, handleSubmit, watch, control } = useForm<Inputs>();
  const [selection, setSelection] = useState<string[]>([]);

  const queryType = watch("mode");
  const filter = watch("filter");
  const data = useLoaderData() as Data;
  // const { tags, performers } = data;

  const { tags, performers } = filterData(data, filter);

  let stage = FormStage.SelectMode;
  if (queryType === "performers" || queryType === "tags") {
    stage = FormStage.SelectCriteria;
  }

  const onSubmit = (values: Inputs) => {
    console.log(values);
  };

  return (
    <section className="py-4 flex flex-col">
      <h1 className="text-4xl font-bold mb-4">Stash Compilation Generator</h1>
      <ul className="steps mb-4">
        <li className="step step-primary">Choose mode</li>
        <li
          className={clsx(
            "step",
            stage >= FormStage.SelectCriteria && "step-primary"
          )}
        >
          Select criteria
        </li>
        <li
          className={clsx(
            "step",
            stage >= FormStage.VideoOptions && "step-primary"
          )}
        >
          Select video options
        </li>
        <li className="step">Wait for video</li>
      </ul>

      <form
        className="flex flex-col items-start gap-4"
        onSubmit={handleSubmit(onSubmit)}
      >
        <p>
          {stage === FormStage.SelectMode &&
            "You can filter markers either by performers or by tags."}
          {stage === FormStage.SelectCriteria &&
            "Select the performers or tags you want to include in your compilation."}
        </p>
        <select
          defaultValue="none"
          className="select select-primary w-full max-w-xs"
          {...register("mode")}
        >
          <option disabled value="none">
            Select query type...
          </option>
          <option value="tags">Tags</option>
          <option value="performers">Performers</option>
        </select>

        {queryType && (
          <div className="w-full flex justify-between">
            <input
              type="text"
              placeholder="Filter..."
              className="input input-bordered"
              {...register("filter")}
            />
            <button
              type="button"
              onClick={() => setSelection([])}
              className="btn btn-error"
            >
              Clear selection
            </button>
          </div>
        )}
        {queryType === "performers" && (
          <section className="grid grid-cols-4 gap-2 w-full">
            {performers.map((performer) => (
              <article
                key={performer.id}
                className="card bg-base-100 shadow-xl"
              >
                <figure>
                  <img
                    src={performer.imageUrl}
                    alt={performer.name}
                    className="aspect-[2/3] object-cover object-top w-full"
                  />
                </figure>
                <div className="card-body">
                  <h2 className="card-title">{performer.name}</h2>
                  <p>{performer.sceneCount} scenes</p>
                  <div className="card-actions justify-end">
                    <div className="form-control">
                      <label className="label cursor-pointer">
                        <span className="label-text">Select</span>
                        <input
                          type="checkbox"
                          className="checkbox checkbox-primary ml-2"
                          checked={selection.includes(performer.id)}
                          onChange={(e) =>
                            setSelection((s) => [...s, performer.id])
                          }
                        />
                      </label>
                    </div>
                  </div>
                </div>
              </article>
            ))}
          </section>
        )}

        {queryType === "tags" && (
          <section className="grid grid-cols-4 gap-2 w-full">
            {tags.map((tag) => (
              <article key={tag.id} className="card bg-base-100 shadow-xl">
                <div className="card-body">
                  <h2 className="card-title">{tag.name}</h2>
                  <p>{tag.count} markers</p>
                  <div className="card-actions justify-end">
                    <div className="form-control">
                      <label className="label cursor-pointer">
                        <span className="label-text">Select</span>
                        <input
                          type="checkbox"
                          className="checkbox checkbox-primary ml-2"
                          checked={selection.includes(tag.id)}
                          onChange={(e) => setSelection((s) => [...s, tag.id])}
                        />
                      </label>
                    </div>
                  </div>
                </div>
              </article>
            ))}
          </section>
        )}
      </form>
    </section>
  );
}

export default App;
