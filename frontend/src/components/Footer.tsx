import ExternalLink from "./ExternalLink"

function Footer({version}: {version: string}) {
  return (
    <footer className="w-full text-center text-sm flex flex-col gap-1 my-4">
      <p>ClipMash v{version}</p>
      <p>
        Made with <span className="text-red-500">❤️</span> by{" "}
        <ExternalLink href="https://soundchaser128.xyz">
          soundchaser128
        </ExternalLink>
        .
      </p>
      <p>
        This project is open source and available on{" "}
        <ExternalLink href="https://github.com/soundchaser128/clip-mash">
          GitHub
        </ExternalLink>
        .
      </p>
    </footer>
  )
}

export default Footer
