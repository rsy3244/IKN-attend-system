<template>
  <div class="attend-list">
    <v-card
      max-width="500"
      class="mx-auto"
    >
      <v-toolbar
        :color="topColor"
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
            <v-list-item v-text="student.role"></v-list-item>
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

  private topColor: string = 'deep-purple lighten-1';
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
        // alert("internal server error");
        this.students[id].state = 0;
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
        // alert("internal server error");
        this.students[id].state = 1;
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
      if (this.room === 'P') {
        return [
          { id: 1, name: '有村博紀', role: '教授', state: 1},
          { id: 2, name: '喜田拓也', role: '准教授', state: 1},
          { id: 3, name: '眞鍋 由布', role: '秘書', state: 1},
        ];
      }
      return [ // 404 の時にこれを返す TODO 後で reutrn [] にする．
        { id: 1, name: '古谷勇', role: 'D1', state: 1},
        { id: 2, name: '加井丈志', role: 'M1', state: 1},
        { id: 3, name: '大泉翼', role: 'B4', state: 1},
        { id: 4, name: '光吉健汰', role: 'B4', state: 0},
        { id: 5, name: '小畠教寛', role: 'B4', state: 1},
        { id: 6, name: '又康太', role: 'B4', state: 1},
        { id: 7, name: '奥地諒太', role: 'B4', state: 1},
      ];
    }).then((json) => {
      this.students = json;
    });
  }
  private created(): void {
    this.getStudents();
    if (this.room !== 'P') {
      this.topColor = 'indigo lighten-1';
    }
  }

}
</script>

<style>
</style>
