import React, { useState, useEffect } from 'react';
import './NotesApp.scss';
import { invoke } from "@tauri-apps/api/tauri";

const NotesApp = (props) => {
  // const {  } = props;

  const [searchTerm, setSearchTerm] = useState("");

  const search = async (searchTerm) => {
    await invoke("search", { searchTerm })
  }

  useEffect(() => {
    if (searchTerm) {
      // hit rust/sqlite
      // search(searchTerm);
    }
  }, [searchTerm]);

  return (
    <div className="notes__app">
      <div className="notes__app-sidebar">
        <input
          type="text"
          value={searchTerm}
          placeholder="search notes"
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <h2>recent notes</h2>
      </div>
      <div className="notes__app-body"></div>
    </div>
  )
}

export default NotesApp;