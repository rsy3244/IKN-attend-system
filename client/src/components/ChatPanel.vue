<template>
  <div class="chat-panel">
    自由に書き込んでください．
    <v-bottom-sheet v-model="sheet" inset persistent>
      <template v-slot:activator="{ on }">
        <v-btn
          dark
          fab
          top
          right
          color="teal darken-1"
          v-on="on"
        >
          <v-icon>mdi-message-text-outline</v-icon>
        </v-btn>
      </template>
      <v-sheet class="text-center" height="200px">
        <v-container fluid>
          <v-row>
            <v-col>
              <v-textarea
                v-model="text"
                label="Text"
              ></v-textarea>
            </v-col>
            <v-col>
              <v-text-field
                v-model="name"
                :counter="10"
                label="Name"
                required
              ></v-text-field>
              <v-btn
                class="mt-6"
                text
                color="teal darken-1"
                outlined
                @click="send"
              >send</v-btn>
              <v-btn
                class="mt-6"
                text
                color="error"
                @click="sheet = !sheet"
              >close</v-btn>
            </v-col>
          </v-row>

        </v-container>
      </v-sheet>
    </v-bottom-sheet>
    <ChatBoard />
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';
import ChatBoard from '@/components/ChatBoard.vue';

@Component({
  components: {
    ChatBoard,
  },
})
export default class ChatPanel extends Vue {
  private sheet: boolean = false;
  private text: string = '';
  private name: string = '';
  private send(): void {
    alert(this.name);
    alert(this.text);
    this.sheet = !this.sheet;
    this.clear();
  }
  private clear(): void {
    this.name = '';
    this.text = '';
  }
}
</script>

<style>
.chat-panel {
  margin: 5%;
}
</style>
