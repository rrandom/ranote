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

import RFC from '../RFC';
import store from '../store';

@Component({
})
export default class Preview extends Vue {

  public result = '';
  public md: null | MarkdownIt = null;

  public updateContent(content: string) {
    this.result = this.md!.render(content);
  }

  public loadNote(noteName: string) {
    window.loadNoteCb = (note: Note) => {
      store.commit('setCurrentNote', note);
      this.updateContent(note.content!);
    };
    RFC.loadNote(noteName);
  }

  @Watch('$route.query')
  public onChange(newV: any) {
    this.loadNote(newV.name);
  }

  public mounted() {
    this.md = new MarkdownIt();
    if (store.state.currentNote) {
      this.updateContent(store.state.currentNote.content!);
    } else {
      if (this.$route.query) {
        this.loadNote(this.$route.query.name as string);
      }
    }
  }
}
</script>
<style lang="scss">
.preview-container {
  padding-left: 10px;
  overflow-y: scroll;
  overflow-x: hidden;
  max-height: 100vh;
  height: 100%;
}
</style>
