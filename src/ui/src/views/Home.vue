<template>
  <div class="home">
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

import CodeMirror from 'codemirror';

import 'codemirror/mode/markdown/markdown';
import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/midnight.css';
import 'codemirror/keymap/emacs';

@Component({
  beforeRouteLeave(to, from, next) {
    let value =  this.cm.getDoc().getValue();
    window.localStorage.setItem('content', value);
    next();
  },
})
export default class Home extends Vue {

  public cm: null | CodeMirror.Editor = null;

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
