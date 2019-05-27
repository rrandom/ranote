<template>
  <div id="app">
    <LeftBar />
    <MiddleBar />
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
<script>
import LeftBar from '@/components/LeftBar';
import MiddleBar from '@/components/MiddleBar';

export default {
  components: {
    LeftBar,
    MiddleBar,
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
