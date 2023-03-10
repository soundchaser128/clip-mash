import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'

function App() {
  const [content, setContent] = useState<string>()

  useEffect(() => {
    fetch("/api/hello")
    .then(res => res.text())
    .then(text => setContent(text))

  }, [])
  
  return (
    <div className="App">
      {content || "Loading..."}
    </div>
  )
}

export default App
