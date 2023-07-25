
import "./App.css";


// SETTINGS WIP

// import { Store } from "tauri-plugin-store-api";

// const store = new Store(".settings.dat");

// await store.set("another-key", { value: 3 });
// await store.save(); // this manually saves the store, otherwise the store is only saved when your app is closed

// const val = await store.get("some-key");
// const val2 = await store.get("another-key");
// console.log(val)
// console.log(val2)

// SETTINGS WIP


//import Login from "./routes/Login"
import Dashboard from "./routes/Dashboard";

const App = () => {
  return (
    <div>
      <Dashboard />
    </div>
  )
}

export default App;
