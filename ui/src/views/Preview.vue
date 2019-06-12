<template>
  <div class="preview-container" v-html="result">
    <h1>This is an preview page</h1>
  </div>
</template>
<script lang="ts">
import { Component, Vue, Watch } from 'vue-property-decorator';
import { Route } from 'vue-router';
import MarkdownIt from 'markdown-it';

import store from '../store';

@Component({
})
export default class Preview extends Vue {

  public result = '';
  public md: null | MarkdownIt = null;

  public updateContent(contents: string) {
    this.result = this.md!.render(contents);
  }

  public mounted() {
    this.md = new MarkdownIt();
    if (store.state.currentNote) {
      this.updateContent(store.state.currentNote.contents);
    }
  }

  @Watch('$store.state.currentNote')
  public onNoteChanged(val: any, oldVal: any) {
    this.updateContent(val.contents);
  }
}
</script>
<style lang="scss">
.preview-container {
  padding-left: 10px;
}
</style>
