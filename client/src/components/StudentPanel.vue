<template>
  <div class="student-panel">
    <v-card
      max-width="500"
      class="mx-auto"
    >
      {{grade}}
      {{name}}
      <span v-if="state === 1">
        <v-btn depressed @click="leave" color="normal">退席中</v-btn>
      </span>
      <span v-else>
        <v-btn depressed @click="attend" color="primary">出席中</v-btn>
      </span>
    </v-card>
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

@Component({
  components: {
  },
})
export default class StudentPanel extends Vue {
  private grade: string = 'B4';
  private name: string = '大泉翼';
  private state: number = 1;

  private leave(): void {
    this.state = 0;
  }
  private attend(): void {
    this.state = 1;
    this.getName();
  }
  private getName(): void {

    const url = '/api/test';
    fetch(url).then((response) => {
      if (response.ok) {
        return response.json();
      }
      return [];
    }).then((json) => {
      this.name = json.name;
    });
  }

}
</script>

<style>
.student-panel {
  margin: 3%;
}

</style>
