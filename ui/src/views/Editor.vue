<template>
  <div class="editor-container" @click="onClick">
  </div>
</template>

<script lang="ts">
import { Component, Vue, Watch } from 'vue-property-decorator';
import { Route } from 'vue-router';
import { Note } from '../types';

import CodeMirror from 'codemirror';
import 'codemirror/mode/markdown/markdown';
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/midnight.css';
import 'codemirror/keymap/emacs';

import store from '../store';
import RFC from '../RFC';

@Component({
})
export default class Editor extends Vue {

  public cm: null | CodeMirror.Editor = null;

  public beforeRouteLeave(to: Route, from: Route, next: any) {
    const value = this.cm!.getDoc().getValue();
    const currentNote = store.state.currentNote!;
    RFC.saveNote(currentNote, value);

    next();
  }

  public onClick() {
    console.log('onClick');
    window.testClickCb = () => {
      // alert('testClickCb called');
    };
    RFC.testClick();
  }

  public loadNote(noteName: string) {
    window.loadNoteCb = (note: Note) => {
      store.commit('setCurrentNote', note);
      this.cm!.setValue(note.content as string);
    };
    RFC.loadNote(noteName);
  }

  @Watch('$route.query')
  public onChange(newV: any, oldV: any) {
    if (store.state.currentNote && store.state.currentNote.name == oldV.name) {
      console.log('saving');
      const value = this.cm!.getDoc().getValue();
      RFC.saveNote(store.state.currentNote, value);
    }

    this.loadNote(newV.name);
  }

  public mounted() {
    const cm = CodeMirror(this.$el as HTMLElement, {
      value: '',
      mode:  'markdown',
      theme: 'midnight',
      lineNumbers: true,
      keyMap: 'emacs',
    });
    this.cm = cm;
    cm.setSize(null, '100%');

    if (this.$route.query.name) {
      this.loadNote(this.$route.query.name as string);
    }
  }
}
</script>
<style lang="scss">
.editor-container {
  height: 100%;
}
</style>

