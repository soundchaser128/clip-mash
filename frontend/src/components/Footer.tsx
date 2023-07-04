function Footer({version}: {version: string}) {
  return (
    <footer className="w-full text-center text-sm flex flex-col gap-1 my-4">
      <p>ClipMash v{version}</p>
      <p>
        Made with ❤️ by{" "}
        <a
          href="https://soundchaser128.xyz"
          className="link"
          target="_blank"
          rel="noreferrer"
        >
          soundchaser128
        </a>
        .
      </p>
      <p>
        This project is open source and available on{" "}
        <a
          className="link"
          href="https://github.com/soundchaser128/stash-compilation-maker"
          target="_blank"
          rel="noreferrer"
        >
          GitHub
        </a>
        .
      </p>
    </footer>
  )
}

export default Footer
