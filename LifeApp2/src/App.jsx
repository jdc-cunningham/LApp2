import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./styles/css-reset.css";
import "./App.css";
import AppTabs from "./components/tabs/Tabs";
import { useEffect } from "react";

const App = () => {
  // shouldn't be here
  const apps = {
    Notes: "Notes",
    Finances: "Finances"
  };

  const [activeApp, setActiveApp] = useState(apps.Notes); // use enums

  useEffect(() => {
    console.log('app render');
  });

  return (
    <div className="container">
      <AppTabs apps={apps} activeApp={activeApp} setActiveApp={setActiveApp}/>
    </div>
  );
}

export default App;
