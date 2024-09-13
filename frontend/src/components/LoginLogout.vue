<template>
    <button @click="toggleLogin">{{ buttonText }}</button>
  </template>
  
  <script lang="ts">
  import { computed, defineComponent } from 'vue'
  import { useStore } from 'vuex'
  
  export default defineComponent({
    setup() {
      const store = useStore()
  
      const isLoggedIn = computed(() => store.getters.isLoggedIn)
      const buttonText = computed(() => isLoggedIn.value ? 'Logout' : 'Login')
  
      const toggleLogin = async () => {
        if (isLoggedIn.value) {
          await store.dispatch('logout')
        } else {
          // For simplicity, we're not handling the login form here.
          // You might want to show a login modal or navigate to a login page.
          console.log('Show login form')
        }
      }
  
      return {
        buttonText,
        toggleLogin
      }
    }
  })
  </script>