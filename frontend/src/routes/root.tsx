import { Outlet } from "react-router-dom";

export default function Root() {
  return (
    <main className="container ml-auto mr-auto w-screen h-screen">
      <Outlet />
    </main>
  );
}
