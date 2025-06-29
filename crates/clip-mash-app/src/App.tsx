import { useEffect, useState } from "react";
import { sidecar } from "./lib/sidecar";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    sidecar.start().catch(console.error);
  }, []);

  return (
    <main className="container">
      <p>Hello</p>
    </main>
  );
}

export default App;
