<template>
  <div class="template-conponent">
    これはテンプレートコンポーネント
  </div>
</template>


<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator';

@Component({
  components: {
  },
})
export default class TemplateComponent extends Vue {
}
</script>

<style>
</style>
