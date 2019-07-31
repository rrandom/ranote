import Vue from 'vue';
import Vuex, { createHelpers } from 'vuex-typescript-interface';
import { Note, ActiveNote } from './types';

Vue.use(Vuex);

interface IStore {
  // State (are non-function properties not marked readonly)
  notes: Note[];
  activeNote: null | ActiveNote;
  // Getters (are non-function properties marked readonly)

  // Mutations (functions return void and optionally accept payload)
  setNotes(notes: Note[]): void;
  setActiveNote(note: ActiveNote): void;
  updateNoteName({name, id}: {name: string, id: string}): void;
}

export default new Vuex.Store<IStore>({
  // if state detected on IStore, this will be required and all state defined
  state: {
    notes: [],
    activeNote: null,
  },
  // if mutations detected on IStore, this will be required and all mutations defined
  mutations: {
    // all arguments are type safe here as well
    setNotes(state, notes) {
      state.notes = notes;
    },
    setActiveNote(state, note) {
      state.activeNote = note;
    },
    updateNoteName(state, {name, id }) {
      state.notes.forEach((note) => {
        if (note.id === id) {
          note.name = name;
        }
      });
    },
  },
});
