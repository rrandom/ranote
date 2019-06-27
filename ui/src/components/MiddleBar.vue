<template>
  <div class="middle-bar">
    <div
      v-for="(note, k) in notes"
      @click="onClickNote(note)"
      :key="k"
    >
      {{ note.name }}
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Vue, Prop, Emit } from 'vue-property-decorator';
import RFC from '../RFC';
import { Note } from '../types';

@Component({
})
export default class MiddleBar extends Vue {
  @Prop() public notes!: Note;

  @Emit()
  public onClickNote(name: Note) {
  }
}
</script>
<style lang="scss">
.middle-bar {
  border-right: #000 solid 1px;
  border-left: #000 solid 1px;
}
</style>
