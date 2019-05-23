<template>
  <div class="about" v-html="result">
    <h1>This is an about page</h1>
  </div>
</template>
<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Markdown from 'markdown-it';

@Component({
  beforeRouteEnter (to, from, next) {
    let content = window.localStorage.getItem('content');
    console.log(content);
    let md = new Markdown();
    let result = md.render(content);
    console.log(result);
    next(vm => {
      vm.result = result;
    });
  }
})
export default class About extends Vue {

  result = '';

  mounted() {
  }

  
}
</script>