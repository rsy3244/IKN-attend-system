<template>
  <div class="home">
    <center>
    <v-row align="center">
      <v-col cols="12" sm="3">
        <AttendList />
      </v-col>
      <v-col cols="12" sm="3">
        <AttendList />
      </v-col>
    </v-row>
    </center>
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

import HelloWorld from '@/components/HelloWorld.vue';
import AttendList from '@/components/AttendList.vue';


@Component({
  components: {
    HelloWorld,
    AttendList,
  },
})
export default class Home extends Vue {
}
</script>

<style>
</style>
