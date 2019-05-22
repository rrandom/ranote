<template>
  <div class="home">
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src

import codemirror from 'codemirror';
// import 'codemirror/mode/javascript/javascript';
import 'codemirror/mode/markdown/markdown';

import 'codemirror/lib/codemirror.css';
import 'codemirror/theme/midnight.css';
import 'codemirror/keymap/emacs';

@Component({
  components: {
    HelloWorld,
  },
})
export default class Home extends Vue {
  public mounted() {
    const k = codemirror(this.$el as HTMLElement, {
      value: 'function myScript(){return 100;}\n',
      mode:  'markdown',
      theme: 'midnight',
      lineNumbers: true,
    });
  }
}
</script>
