import Greet from "./greet";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1>Hello from Tauri lol</h1>
      <button className="border">Click here</button>
      <Greet />
    </main>
  );
}
