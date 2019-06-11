import Vue from 'vue';
import Vuex, { createHelpers } from 'vuex-typescript-interface';

Vue.use(Vuex);

interface Note {
  name: string;
  contents: string;
}

interface IStore {
  // State (are non-function properties not marked readonly)
  notes: string[];
  currentNote: null | Note;
  // Getters (are non-function properties marked readonly)

  // Mutations (functions return void and optionally accept payload)
  setNotes(notes: string[]): void;
  setCurrentNote(note: Note): void;
}

export default new Vuex.Store<IStore>({
  // if state detected on IStore, this will be required and all state defined
  state: {
    notes: [],
    currentNote: null,
  },
  // if mutations detected on IStore, this will be required and all mutations defined
  mutations: {
    // all arguments are type safe here as well
    setNotes(state, notes) {
      state.notes = notes;
    },
    setCurrentNote(state, note) {
      state.currentNote = note;
    },
  },
});
