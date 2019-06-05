<template>
  <div class="home" @click="onClick">
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Route } from 'vue-router';

import CodeMirror from 'codemirror';
import 'codemirror/mode/markdown/markdown';
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/midnight.css';
import 'codemirror/keymap/emacs';

import RFC from '../RFC';

@Component({
})
export default class Editor extends Vue {

  public cm: null | CodeMirror.Editor = null;

  public beforeRouteLeave(to: Route, from: Route, next: any) {
    const value = this.cm!.getDoc().getValue();
    window.localStorage.setItem('content', value);
    next();
  }

  public onClick() {
    // TO-DO
    (window as any).testClickCb = () => {
      alert('testClickCb called');
    };
    RFC.testClick();
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
  }
}
</script>
