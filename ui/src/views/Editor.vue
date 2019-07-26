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
    if (store.state.currentNote && store.state.currentNote.name === oldV.name) {
      const value = this.cm!.getDoc().getValue();
      RFC.saveNote(store.state.currentNote, value);
    }

    RFC.debug({o: 'onChange', i: newV});
    this.loadNote(newV.id);
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

    RFC.debug(this.$route.query);

    if (this.$route.query.id) {
      this.loadNote(this.$route.query.id as string);
    }
  }
}
</script>
<style lang="scss">
.editor-container {
  height: 100%;
}
</style>

