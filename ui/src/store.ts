import Vue from 'vue';
// import Vuex from 'vuex';
import Vuex from 'vuex-typescript-interface';

Vue.use(Vuex);
interface IStore {
  // State (are non-function properties not marked readonly)
  notes: string[];
  // Getters (are non-function properties marked readonly)

  // Mutations (functions return void and optionally accept payload)
  setNotes(notes: string[]): void;
}

export default new Vuex.Store<IStore>({
  // if state detected on IStore, this will be required and all state defined
  state: {
    notes: [],
  },
  // if mutations detected on IStore, this will be required and all mutations defined
  mutations: {
    // all arguments are type safe here as well
    setNotes(state, notes) {
      state.notes = notes;
    },
  },
});
