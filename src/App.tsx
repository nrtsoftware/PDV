import { useState } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import Database from "tauri-plugin-sql-api";

// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
const db = await Database.load("sqlite:app.vaitomarnocustyles");
// mysql
// const db = await Database.load("mysql://user:pass@host/database");
// postgres
// const db = await Database.load("postgres://postgres:password@localhost/test");

// await db.execute("INSERT INTO ...");

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
    async function database() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("database_test").then(e => console.log(JSON.parse(e))).catch(e => console.log(e, 'maizenas erradas'));
  }
  database();
  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }
  return (
    <div className="flex flex-col text-center ">
      <h1 className="text-4xl font-bold">CRM</h1>

      <div className="flex justify-center">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="h-10 mx-5 logo vite" alt="Vite logo" />
        </a>
      </div>
      { name }
      <form className="flex flex-col">
        <input className="rounded my-2 h-10 p-3 text-gray-600 text-semibold"
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Usuário"
        />

        <input type="password" className="rounded my-2 h-10 p-3 text-gray-600 text-semibold"
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Senha"
        />

      <button className="rounded m-5 p-3 btn btn-blue ease-in-out transition duration-350 border-2
        text-gray-300 font-semibold
        bg-transparent  border-gray-500
        hover:bg-neutral-600 hover:text-white hover:border-black" 
        type="submit">Entrar</button>
      </form>
      
      <span className="fixed bottom-5 right-5 flex text-lg">
        <p className="font-bold pr-1">NRT</p> <p>Software®</p>
      </span>
      
    </div>
  );
}

export default App;
