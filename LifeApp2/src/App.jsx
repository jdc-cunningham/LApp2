import { useState, useEffect } from "react";
import "./styles/css-reset.css";
import "./App.css";
import AppTabs from "./components/tabs/Tabs";
import NotesApp from "./apps/notes/NotesApp";

const App = () => {
  // shouldn't be here
  const apps = {
    Notes: "Notes",
    Finances: "Finances"
  };

  const [activeApp, setActiveApp] = useState(apps.Notes); // use enums

  const activeAppBody = <NotesApp/>;

  useEffect(() => {
    console.log('app render');
  });

  return (
    <div className="container">
      <AppTabs apps={apps} activeApp={activeApp} setActiveApp={setActiveApp}/>
      {activeAppBody}
    </div>
  );
}

export default App;
