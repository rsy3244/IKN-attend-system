<template>
  <div class="home">
    <v-row>
      <v-col>
        <AttendList 
          room='D'
        />
      </v-col>
      <v-col>
        <AttendList
          room='S'
        />
      </v-col>
    </v-row>
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
