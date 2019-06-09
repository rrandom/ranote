<template>
  <div id="app">
    <LeftBar/>
    <MiddleBar :notes="notes" />
    <div class="editor-panel">
      <div id="nav">
        <router-link to="/">Editor</router-link> |
        <router-link to="/preview">Preview</router-link>
      </div>
      <keep-alive>
        <router-view/>
      </keep-alive>
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import RFC from '@/RFC.ts';
import LeftBar from '@/components/LeftBar.vue';
import MiddleBar from '@/components/MiddleBar.vue';

@Component({
  components: {
    LeftBar,
    MiddleBar,
  },
})
export default class App extends Vue {
  public notes = [];
  public mounted() {
    window.listDir = (notes: any) => {
      console.log('listDir: ', notes);
      this.notes = notes;
    };
    console.log('App mounted');
    RFC.init();
  }
}
</script>
<style lang="scss">
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;

  display: flex;
  flex-direction: row;
}


#nav {
  padding: 30px;
  a {
    font-weight: bold;
    color: #2c3e50;
    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
