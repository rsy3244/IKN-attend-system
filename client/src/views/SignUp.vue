<template>
  <div class="signup">
    <form>
      <v-text-field
        v-model="name"
        :error-messages="nameErrors"
        :counter="10"
        label="Name"
        required
        @input="$v.name.$touch()"
        @blur="$v.name.$touch()"
      ></v-text-field>
      <v-select
        v-model="role"
        :items="roleList"
        :error-messages="selectErrors"
        label="Item"
        required
        @change="$v.select.$touch()"
        @blur="$v.select.$touch()"
      ></v-select>

      <v-btn class="mr-4" @click="signup">Sign Up</v-btn>
      <v-btn @click="clear">clear</v-btn>
    </form>
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

@Component({
  components: {
  },
})
export default class SignUp extends Vue {
  private name: string = '';
  private role: string = '';
  private roleList: string[] = [
    'B1',
    'B2',
    'B3',
    'B4',
    'M1',
    'M2',
    'D1',
    'D2',
    'D3',
    'Secretary',
    'Professor',
    'AssociateProfessor',
    'ResearchAssociate',
  ];
  private signup(): void {
    const url = 'api/signup';
    const method = 'POST';
    const body = JSON.stringify({
      name: this.name,
      role: this.role,
    });
    fetch(url, {method, body}).then((response) => {
      if (response.ok) {
        return response.json();
      }
      alert('登録できませんでした');
    }).then((json) => {
      if (typeof json === 'undefined') {
        return;
      }
    });
  }
  private clear(): void {
    this.name = '';
    this.role = '';
  }
}
</script>

<style>
.signup {
  margin: 10%;
}
</style>
