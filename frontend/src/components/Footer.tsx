const Footer = () => {
  return (
    <footer className="w-full text-center text-sm font-light flex flex-col gap-1 my-4">
      <p>
        Made by{" "}
        <a href="https://soundchaser128.xyz" className="link">
          soundchaser128
        </a>
      </p>
      <p>
        This project is open source and available on{" "}
        <a
          className="link"
          href="https://github.com/soundchaser128/stash-compilation-maker"
        >
          GitHub
        </a>
      </p>
    </footer>
  )
}

export default Footer
