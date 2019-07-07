<template>
  <div id="app">
    <LeftBar/>
    <MiddleBar
      :notes="notes"
      @on-click-note="onClickNote"
      @on-new-note="onNewNote"
    />
    <div class="editor-panel">
      <div class="menu-bar">
        <router-link to="/">Editor</router-link> |
        <router-link to="/preview">Preview</router-link>
      </div>
      <keep-alive>
        <router-view/>
      </keep-alive>
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import RFC from './RFC';
import LeftBar from '@/components/LeftBar.vue';
import MiddleBar from '@/components/MiddleBar.vue';
import { Note } from './types';
import store from './store';

@Component({
  components: {
    LeftBar,
    MiddleBar,
  },
})
export default class App extends Vue {

  public getNotesAndLoad() {
    window.listDir = (notes: Note[]) => {
      console.log('notes', notes);
      store.commit('setNotes', notes);
    };
    window.initCb = window.listDir;
    console.log('App mounted');
    RFC.init();
  }

  public mounted() {
    this.getNotesAndLoad();
  }

  get notes() {
    return store.state.notes;
  }

  public onClickNote(note: Note) {
    window.loadNoteCb = (note: Note) => {
      console.log(note);
      store.commit('setCurrentNote', note);
    };
    RFC.loadNote(note);
  }

  public onNewNote() {
    window.newNoteCb = (note: Note) => {
      this.getNotesAndLoad();
      console.log('newnote', note);
      this.onClickNote(note);
    };
    RFC.newNote();
  }
}
</script>
<style lang="scss">
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;

  display: flex;
  flex-direction: row;
  height: 95vh;
  box-sizing: border-box;
  overflow: hidden;
}

.menu-bar {
  padding: 5px;
  height: 30px;
  font-size: 146x;
  a {
    font-weight: bold;
    color: #2c3e50;
    &.router-link-exact-active {
      color: burlywood;
    }
  }
}

.editor-panel {
  flex: 1 1;
  height: calc(95vh - 30px);
}
</style>
