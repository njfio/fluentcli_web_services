<template>
  <transition
    :name="transitionName"
    :mode="mode"
    @before-enter="beforeEnter"
    @after-enter="afterEnter"
    @enter-cancelled="enterCancelled"
  >
    <router-view v-slot="{ Component }">
      <component :is="Component" />
    </router-view>
  </transition>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'RouteTransition',
  props: {
    transitionName: {
      type: String,
      default: 'fade',
    },
    mode: {
      type: String as () => 'out-in' | 'in-out' | undefined,
      default: 'out-in',
    },
  },
  methods: {
    beforeEnter() {
      // Add any pre-animation setup here
    },
    afterEnter() {
      // Add any post-animation cleanup here
    },
    enterCancelled() {
      // Handle cancelled enter transitions here
    },
  },
});
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>