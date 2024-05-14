import {useRef} from "react"

const suggestions = [
  "Blowjob",
  "Doggy Style",
  "Missionary",
  "Reverse Cowgirl",
  "Cowgirl",
  "Anal",
  "Titty Fucking",
  "Masturbation",
]

const ClipMashTv: React.FC = () => {
  const videoRef = useRef<HTMLVideoElement>(null)

  return (
    <main className="w-full h-screen flex flex-row">
      <section>
        <video className="w-full h-full" ref={videoRef} />
      </section>
      <section>
        <h1 className="text-4xl font-bold">ClipMash TV</h1>
      </section>
    </main>
  )
}

export default ClipMashTv
