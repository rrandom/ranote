<template>
  <div class="preview-container">
    <div v-html="result"></div>
    <!-- <h1>This is an preview page</h1> -->
  </div>
</template>
<script lang="ts">
import { Component, Vue, Watch } from 'vue-property-decorator';
import { Route } from 'vue-router';
import MarkdownIt from 'markdown-it';
import { Note } from '../types';

import store from '../store';

@Component({
})
export default class Preview extends Vue {

  public result = '';
  public md: null | MarkdownIt = null;

  public updateContent(content: string) {
    this.result = this.md!.render(content);
  }

  public mounted() {
    this.md = new MarkdownIt();
    if (store.state.currentNote) {
      this.updateContent(store.state.currentNote.content!);
    }
  }

  @Watch('$store.state.currentNote')
  public onNoteChanged(note: Note) {
    this.updateContent(note.content!);
  }
}
</script>
<style lang="scss">
.preview-container {
  padding-left: 10px;
  overflow: scroll;
  max-height: 100vh;
}
</style>
