import React, { useState, useEffect } from 'react';
import './NotesApp.scss';
import { invoke } from "@tauri-apps/api/tauri";

const NotesApp = (props) => {
  // const {  } = props;

  const [searchTerm, setSearchTerm] = useState("");
  const [searchResults, setSearchResults] = useState([]);
  const [openedNotes, setOpenedNotes] = useState([]);

  const search = async (searchTerm) => {
    await invoke("search", { searchTerm })
  }

  const updateNote = (noteId, noteName, noteBody) => {
    setOpenedNotes(prevState => ([
      ...prevState.filter(note => note.id !== noteId),
      {
        id: noteId,
        name: noteName,
        body: noteBody
      },
    ]));
  }

  useEffect(() => {
    console.log(openedNotes);
  }, [openedNotes]);

  useEffect(() => {
    if (searchTerm) {
      setSearchResults([
        {id: 1, name: "apple", body: "body 1"},
        {id: 2, name: "atmosphere", body: "body 2"}
      ])
      // hit rust/sqlite
      // search(searchTerm);
    } else {
      setSearchResults([]);
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
        {(searchResults.length > 0) && <div className="notes__app-sidebar-search-results">
          {searchResults.map(searchResult =>
            <div
              key={searchResult.id}
              className="notes__app-sidebar-search-result"
              onClick={() => setOpenedNotes(prevState => ([
                ...prevState,
                searchResult,
              ]))}
            >
              {searchResult.name}  
            </div>
          )}
        </div>}
        {!searchResults.length && <h2>recent notes</h2>}
      </div>
      <div className="notes__app-body">
        {openedNotes.length > 0 && openedNotes.map(openedNote => (
          <div key={openedNote.id} className="notes__app-body-note">
            <textarea
              value={openedNote.body}
              onChange={(e) => updateNote(openedNote.id, openedNote.name, e.target.value)}
            />
          </div>
        ))}
      </div>
    </div>
  )
}

export default NotesApp;