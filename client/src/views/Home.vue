<template>
  <div class="home">
    <center>
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
        <v-col>
          <AttendList
            room='P'
          />
        </v-col>
      </v-row>
      <ChatPanel />
    </center>
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

import HelloWorld from '@/components/HelloWorld.vue';
import AttendList from '@/components/AttendList.vue';
import ChatPanel from '@/components/ChatPanel.vue';


@Component({
  components: {
    HelloWorld,
    AttendList,
    ChatPanel,
  },
})
export default class Home extends Vue {
}
</script>

<style>
.home {
  margin: 5%;
}
</style>
