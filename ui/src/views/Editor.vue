<template>
  <div class="editor-container" @click="onClick">
  </div>
</template>

<script lang="ts">
import { Component, Vue, Watch } from 'vue-property-decorator';
import { Route } from 'vue-router';

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

    store.commit('setCurrentNote', {
      ...currentNote,
      contents: value,
    });

    RFC.saveFile(name, value);

    next();
  }

  public onClick() {
    console.log('onClick');
    window.testClickCb = () => {
      // alert('testClickCb called');
    };
    RFC.testClick();
  }

  @Watch('$store.state.currentNote')
  public onNoteChanged(val: any, oldVal: any) {
    this.cm!.setValue(val.contents);
  }

  public mounted() {
    const cm = CodeMirror(this.$el as HTMLElement, {
      value: 'function myScript(){return 100;}\n',
      mode:  'markdown',
      theme: 'midnight',
      lineNumbers: true,
      keyMap: 'emacs',
    });
    this.cm = cm;
    cm.setSize(null, '100%');
  }
}
</script>
<style lang="scss">
.editor-container {
  height: 100%;
}
</style>

