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

        <v-toolbar-title>
          {{ room }}部屋
        </v-toolbar-title>

        <v-spacer></v-spacer>

      </v-toolbar>
      <v-list>
        <!-- TODO roomid 実装されたら，ここを roomid する -->
        <v-list-item
          v-for="(student, index) in students"
          :key="index"
          @click="switchAtendance(index)"
          v-if="isSameRoom(student.id)"
        >
          <v-list-item-icon>
            <v-icon v-if="student.state === 1" color="pink">mdi-account-circle</v-icon>
          </v-list-item-icon>
            <v-list-item-content>
              <v-list-item v-text="student.role"></v-list-item>
            </v-list-item-content>

            <v-list-item-content>
              <v-list-item-title v-text="student.username"></v-list-item-title>
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
  private students: any = [];
  @Prop()
  private room!: string;
  private isSameRoom(id: number): boolean {
    return true;
  }
  private switchAtendance(id: number): void {
    if (this.students[id].state === 1) {
      this.leave(this.students[id].id);
    } else {
      this.attend(this.students[id].id);
    }
  }
  private leave(id: number): void {
    const url = 'api/leave/' + `${id}`;
    fetch(url, {method: 'PUT'}).then((response) => {
      if (response.ok) {
        this.getStudents();
      } else {
        alert('internal server error');
      }
    });
  }
  private attend(id: number): void {
    const url = 'api/attend/' + `${id}`;
    fetch(url, {method: 'PUT'}).then((response) => {
      if (response.ok) {
        this.getStudents();
      } else {
        alert('internal server error');
      }
    });
  }
  private getStudents(): void {
    const url = 'api/students';
    fetch(url).then((response) => {
      if (response.ok) {
        return response.json();
      }
      if (this.room === 'P') {
        return [
          { id: 1, username: '有村博紀', role: '教授', state: 1},
          { id: 2, username: '喜田拓也', role: '准教授', state: 1},
          { id: 3, username: '眞鍋 由布', role: '秘書', state: 1},
        ];
      }
      if (this.room === 'D') {
        return [ // 404 の時にこれを返す TODO 後で reutrn [] にする．
          { id: 1, username: '栗田和宏', role: 'D3', state: 1},
          { id: 1, username: '金森 憲太朗', role: 'M2', state: 1},
          { id: 1, username: '松田祐汰', role: 'M1', state: 1},
          { id: 1, username: '瀧澤涼介', role: 'M1', state: 1},
          { id: 3, username: '大泉翼', role: 'B4', state: 1},
          { id: 5, username: '小畠教寛', role: 'B4', state: 1},
          { id: 7, username: '奥地諒太', role: 'B4', state: 1},
        ];
      }
      if (this.room === 'S') {
        return [ // 404 の時にこれを返す TODO 後で reutrn [] にする．
          { id: 1, username: '古谷勇', role: 'D1', state: 1},
          { id: 2, username: '鳥谷部直弥', role: 'M2', state: 1},
          { id: 3, username: '山岸大騎', role: 'M2', state: 1},
          { id: 2, username: '加井　丈志', role: 'M1', state: 1},
          { id: 2, username: '三上辰巳', role: 'M1', state: 1},
          { id: 2, username: '王叶', role: 'M1', state: 1},
          { id: 2, username: '三上辰巳', role: 'M1', state: 1},
          { id: 4, username: '光吉健汰', role: 'B4', state: 0},
          { id: 6, username: '又康太', role: 'B4', state: 1},
        ];
      }

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
