<template>
  <div class="preview-container" v-html="result">
    <h1>This is an preview page</h1>
  </div>
</template>
<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Route } from 'vue-router';
import Markdown from 'markdown-it';

import store from '../store';

@Component({
})
export default class Preview extends Vue {

  public result = '';

  public mounted() {
  }

  public beforeRouteEnter(to: Route, from: Route, next: any) {
    const content = store.state.currentNote.contents;

    console.log(content);
    const md = new Markdown();
    const result = md.render(content as string);
    console.log(result);
    next((vm: Preview) => {
      vm.result = result;
    });
  }
}
</script>
<style lang="scss">
.preview-container {
  padding-left: 10px;
}
</style>
