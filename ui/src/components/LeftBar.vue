<template>
  <div class="left-bar">
    <div class="item">
      left bar
    </div>
  </div>
</template>
<script>
export default {
  methods: {
  },
};
</script>
<style lang="scss">
.left-bar {
  // border-right: 1px solid bisque;
  background-color: #20272c;
  color: #fff;
  min-width: 150px;
  text-align: center;
  user-select: none;

  .item {
    text-overflow: ellipsis;
    white-space: nowrap;
    height: 32px;
    padding: 7px 12px;
    box-sizing: border-box;
    cursor: pointer;
    overflow: hidden;
  }
}
</style>
