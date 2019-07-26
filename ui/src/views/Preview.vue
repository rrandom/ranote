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
import { Note, ActiveNote } from '../types';

import Typography from 'typography';
import theme from 'typography-theme-grand-view';
import RFC from '../RFC';
import store from '../store';

theme.overrideThemeStyles = () => ({
  pre: {
    background: '#f8f8f8',
    padding: '3px',
    border: '1px solid #ccc',
    borderRadius: '4px',
  },
  code: {
  },
});

const typography = new Typography(theme);

typography.injectStyles();

@Component({
})
export default class Preview extends Vue {

  public result = '';
  public md: null | MarkdownIt = null;

  public updateContent(content: string) {
    this.result = this.md!.render(content);
  }

  public loadNote(noteName: string) {
    window.loadNoteCb = (note: ActiveNote) => {
      store.commit('setActiveNote', note);
      this.updateContent(note.content);
    };
    RFC.loadNote(noteName);
  }

  @Watch('$route.query')
  public onChange(newV: any) {
    this.loadNote(newV.id);
  }

  public mounted() {
    this.md = new MarkdownIt();
    if (store.state.activeNote) {
      this.updateContent(store.state.activeNote.content);
    } else {
      if (this.$route.query) {
        this.loadNote(this.$route.query.id as string);
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
