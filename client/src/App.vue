<template>
  <v-app>
    <v-app-bar
      app
      dark
      color="blue-grey darken-3"
    >
      <v-toolbar-title>
        <v-btn
          text
          to="/"
        >
          <span class="mr-2">
            IKN-attend-system
          </span>
        </v-btn>
      </v-toolbar-title>

      <v-spacer></v-spacer>
      <v-btn
        text
        to="signup"
      >
        <span class="mr-2">登録</span>
      </v-btn>
    </v-app-bar>

    <v-content>
      <v-container fluid>
        <router-view></router-view>
      </v-container>
    </v-content>
  </v-app>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

@Component({
  components: {
  },
})
export default class App extends Vue {
}
</script>
