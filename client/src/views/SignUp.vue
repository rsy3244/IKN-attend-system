<template>
  <div class="signup">
    これはテンプレートコンポーネント
    conponent
    <div class="my-2" @click="signup">
      <v-btn small>登録</v-btn>
    </div>
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

@Component({
  components: {
  },
})
export default class SignUp extends Vue {
  private signup(): void {
    alert('登録！！');
  }
}
</script>

<style>
</style>
