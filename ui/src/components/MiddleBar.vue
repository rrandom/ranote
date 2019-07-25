<template>
  <div class="middle-bar">
    <div class="tool-bar">
      <div @click="onNewNote" class="tool-bar-btn btn">+</div>
    </div>
    <div class="notes">
      <div
        v-for="(note, k) in notes"
        @click="onClickNote(note)"
        :key="k"
        :class="['note', $route.query.name == note.name ? 'current' : '']"
      >
        {{ note.name }}
      </div>
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

  @Emit()
  public onNewNote() {
  }
}
</script>
<style lang="scss">
.middle-bar {
  border-right: #dedede solid 1px;
  width: 200px;
  user-select: none;

  .tool-bar {
    background-color: #f5f5f5;
    border-color: #d6d6d6;
    color: #505050;
    text-align: center;
    height: 38px;
    border-bottom-width: 1px;
    border-bottom-style: solid;
    display: flex;
    justify-content: center;
    align-items: center;

    &-btn {
    }
  }

  .note {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
    height: 32px;
    box-sizing: border-box;
    padding: 7px 12px;
    margin: 0;
    border-radius: 0;
    width: 100%;
    cursor: pointer;
    overflow: hidden;
    justify-content: left;

    &.current {
      // background: red;
      background-color: rgba(0,0,0,.08);
    }
  }
}
</style>
