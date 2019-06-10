<template>
  <div class="middle-bar">
    <div
      v-for="(note, k) in notes"
      @click="onClickNote(note)"
      :key="k"
    >
      {{ note }}
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import RFC from '../RFC';

@Component({
})
export default class MiddleBar extends Vue {
  @Prop() public notes!: string;

  public onClickNote(name: string) {
    RFC.loadFile(name);
  }
}
</script>
<style lang="scss">
.middle-bar {
  border-right: #000 solid 1px;
  border-left: #000 solid 1px;
}
</style>
