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
        <a @click="toogleState" :class="{active: routerName == 'editor', btn: true}">E</a>
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
import './assets/main.scss';

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

      if (notes.length > 0) {
        this.onClickNote(notes[0]);
      }
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

  get routerName() {
    return this.$route.name;
  }

  public onClickNote(note: Note) {
    this.$router.push({ name: this.$route.name, query: { name: note.name } });
  }

  public onNewNote() {
    window.newNoteCb = (note: Note) => {
      this.getNotesAndLoad();
      this.onClickNote(note);
    };
    RFC.newNote();
  }

  public toogleState() {
    if (this.routerName === 'editor') {
      this.toPreview();
    } else {
      this.toEditor();
    }
  }

  public toEditor() {
    this.$router.push({ name: 'editor', query: this.$route.query });
  }

  public toPreview() {
    this.$router.push({ name: 'preview', query: this.$route.query });
  }
}
</script>
<style lang="scss">
html, body {
  margin: 0;
}

#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;

  display: flex;
  flex-direction: row;
  height: 100vh;
  box-sizing: border-box;
  overflow: hidden;
}

.menu-bar {
  background-color: #f5f5f5;
  border-color: #d6d6d6;
  color: #505050;
  text-align: center;
  height: 38px;
  border-bottom-width: 1px;
  border-bottom-style: solid;
  display: flex;
  justify-content: left;
  align-items: center;
  padding-left: 10px;

  .btn {

    &.active {
      color: #ef6c00;
    }
  }
}

.editor-panel {
  flex: 1 1;
  height: calc(100vh - 30px);
}
</style>
