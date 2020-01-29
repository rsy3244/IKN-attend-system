<template>
  <v-app>
    <v-app-bar
      app
      dark
    >
      <div class="d-flex align-center">
        <span class="mr-6">ikn-attend-system</span>
      </div>

      <v-spacer></v-spacer>
    </v-app-bar>

    <v-content>
      <Home />
    </v-content>
  </v-app>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';
import Home from './views/Home.vue';

@Component({
  components: {
    Home,
  },
})
export default class AttendList extends Vue {
}
</script>
