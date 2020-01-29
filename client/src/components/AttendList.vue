<template>
  <div class="attend-list">
    <center>
      <StudentPanel />
      <StudentPanel />
      <StudentPanel />
      <StudentPanel />
      <StudentPanel />
      <StudentPanel />
    </center>
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';
import StudentPanel from '@/components/StudentPanel.vue';

@Component({
  components: {
    StudentPanel,
  },
})
export default class AttendList extends Vue {
}
</script>

<style>
</style>
