<template>
  <div id="app">
    <LeftBar/>
    <MiddleBar :notes="notes" />
    <div class="editor-panel">
      <div class="menu-bar">
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
import RFC from './RFC';
import LeftBar from '@/components/LeftBar.vue';
import MiddleBar from '@/components/MiddleBar.vue';

@Component({
  components: {
    LeftBar,
    MiddleBar,
  },
})
export default class App extends Vue {
  public mounted() {
    window.listDir = (notes: any) => {
      console.log('notes', notes);
      this.$store.commit('setNotes', notes);
    };
    console.log('App mounted');
    RFC.init();
  }

  get notes() {
    return this.$store.state.notes;
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
  height: 95vh;
  box-sizing: border-box;
  overflow: hidden;
}


.menu-bar {
  padding: 5px;
  height: 30px;
  font-size: 146x;
  a {
    font-weight: bold;
    color: #2c3e50;
    &.router-link-exact-active {
      color: burlywood;
    }
  }
}

.editor-panel {
  flex: 1 1;
  height: calc(95vh - 30px);
}
</style>
