<template>
  <div>
    <div
      v-for="note in notes"
      @click="onClickNote(note)"
    >
      {{ note }}
    </div>
  </div>
</template>
<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import RFC from '@/RFC.ts';

@Component({
})
export default class MiddleBar extends Vue {
  @Prop() public notes!: string;

  public onClickNote(name) {
    RFC.loadFile(name);
  }
}
</script>
