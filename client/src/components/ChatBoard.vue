<template>
  <div class="chat-board">
    <v-card
      max-width="450"
      class="mx-auto"
    >
      <v-toolbar
        color="cyan"
        dark
      >
        <v-app-bar-nav-icon></v-app-bar-nav-icon>

        <v-toolbar-title>Inbox</v-toolbar-title>

        <v-spacer></v-spacer>

        <v-btn icon>
          <v-icon>mdi-magnify</v-icon>
        </v-btn>
      </v-toolbar>

      <v-list three-line>
        <template v-for="(item, index) in items">
          <v-divider
            v-if="item.divider"
            :key="index"
            :inset="item.inset"
          ></v-divider>

          <v-list-item
            v-else
            :key="item.title"
            @click=""
          >
            <v-list-item-content>
              <v-list-item-content>
                {{item.name}}
                {{item.text}}
              </v-list-item-content>
            </v-list-item-content>
          </v-list-item>
        </template>
      </v-list>
    </v-card>
  </div>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

@Component({
  components: {
  },
})
export default class ChatBoard extends Vue {
  private text: string = '食堂いこ';
  private name: string = 'furuya';

  private items: any = [
    {
      name: 'oizumi',
      text: '食堂行こう',
    },
  ];
}
</script>

<style>
.chat-board{
  margin: 5%;
}
</style>
