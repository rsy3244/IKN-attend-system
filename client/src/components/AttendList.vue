<template>
  <div class="attend-list">
    <v-card
      max-width="500"
      class="mx-auto"
    >
      <v-toolbar
        color="indigo lighten-1"
        dark
      >
        <v-app-bar-nav-icon></v-app-bar-nav-icon>

        <v-toolbar-title>
          {{ room }}部屋
        </v-toolbar-title>

        <v-spacer></v-spacer>

        <v-btn icon>
          <v-icon>mdi-magnify</v-icon>
        </v-btn>

        <v-btn icon>
          <v-icon>mdi-dots-vertical</v-icon>
        </v-btn>
      </v-toolbar>
      <v-list>
        <v-list-item
          v-for="(student, index) in students"
          :key="index"
          @click="switchAtendance(index)"
        >
        <v-list-item-icon>
          <v-icon v-if="student.state === 1" color="pink">mdi-account-circle</v-icon>
        </v-list-item-icon>
          <v-list-item-content>
            <v-list-item v-text="student.grade"></v-list-item>
          </v-list-item-content>

          <v-list-item-content>
            <v-list-item-title v-text="student.name"></v-list-item-title>
          </v-list-item-content>

          <v-list-item-content>
            <v-list-item v-if="student.state === 1">在室</v-list-item>
            <v-list-item v-if="student.state === 0">不在</v-list-item>
          </v-list-item-content>

        </v-list-item>
      </v-list>
    </v-card>
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
  @Prop()
  private room!: string;
  private switchAtendance(id: number): void {
    if (this.students[id].state === 1) {
      this.leave(id);
    } else {
      this.attend(id);
    }
  }
  private leave(id: number): void {
    const url = 'api/leave/' + `${id}`;
    fetch(url).then((response) => {
      if (response.ok) {
        // TODO ここは，getStudent を呼び直す仕様にする
        this.students[id].state = 0;
      } else {
        alert("internal server error");
      }
    });
  }
  private attend(id: number): void {
    const url = 'api/leave/' + `${id}`;
    fetch(url).then((response) => {
      if (response.ok) {
        // TODO ここは，getStudent を呼び直す仕様にする
        this.students[id].state = 1;
      } else {
        alert("internal server error");
      }
    });
  }
  private students: any = [];
  private getStudents(): void {
    const url = 'api/students';
    fetch(url).then((response) => {
      if (response.ok) {
        return response.json();
      }
      return [ // 404 の時にこれを返す TODO 後で reutrn [] にする．
        { id: 1, name: '大泉翼', grade: 'B4', state: 1},
        { id: 2, name: '光吉健汰', grade: 'B4', state: 0},
        { id: 3, name: '小畠教寛', grade: 'B4', state: 1},
      ];
    }).then((json) => {
      this.students = json;
    });
  }
  private created(): void {
    this.getStudents();
  }

}
</script>

<style>
</style>
