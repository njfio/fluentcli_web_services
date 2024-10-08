<template>
  <div id="app" class="min-h-screen bg-app text-content flex flex-col">
    <template v-if="isInitialized">
      <template v-if="isLoggedIn">
        <StudioLayout>
          <Studio />
        </StudioLayout>
      </template>
      <template v-else>
        <header class="bg-surface shadow-sm">
          <nav class="studio-container py-3 flex justify-between items-center">
            <router-link to="/" class="text-xl font-bold text-primary">FluentCLI Web Services</router-link>
          </nav>
        </header>
        <main class="flex-grow studio-container py-8 bg-pattern">
          <RouteTransition>
            <router-view></router-view>
          </RouteTransition>
        </main>
      </template>
    </template>
    <div v-else class="flex-grow flex items-center justify-center">
      <div class="loading-spinner"></div>
    </div>
    <footer class="bg-surface border-t border-divider">
      <div class="studio-container py-4 text-center text-content-light">
        &copy; {{ new Date().getFullYear() }} FluentCLI Web Services. All rights reserved.
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import { useRouter } from 'vue-router';
import AuthService from './services/AuthService';
import Studio from './views/Studio.vue';
import RouteTransition from './components/RouteTransition.vue';
import StudioLayout from './components/studio/StudioLayout.vue';

const store = useStore();
const router = useRouter();
const isInitialized = ref(false);

const isLoggedIn = computed(() => store.state.isLoggedIn);

onMounted(async () => {
  const token = AuthService.getToken();
  if (token) {
    try {
      const user = await AuthService.validateToken(token);
      store.commit('setLoggedIn', true);
      store.commit('setUser', user);
      if (router.currentRoute.value.path === '/login') {
        router.push('/studio/dashboard');
      }
    } catch (error) {
      console.error('Token validation failed:', error);
      AuthService.removeToken();
      store.commit('setLoggedIn', false);
      store.commit('setUser', null);
      router.push('/login');
    }
  } else {
    if (router.currentRoute.value.path !== '/login') {
      router.push('/login');
    }
  }
  isInitialized.value = true;
});
</script>

<style>
@import './styles/studio.css';

#app {
  font-family: 'Inter', sans-serif;
}

.bg-app {
  background-color: var(--neutral-100);
}

.bg-surface {
  background-color: var(--neutral-50);
}

.text-content {
  color: var(--neutral-800);
}

.text-content-light {
  color: var(--neutral-600);
}

.text-primary {
  color: var(--primary-600);
}

.border-divider {
  border-color: var(--neutral-200);
}

.loading-spinner {
  border: 4px solid var(--neutral-200);
  border-top: 4px solid var(--primary-600);
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.bg-pattern {
  background-color: var(--neutral-50);
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 100 100'%3E%3Cg fill-rule='evenodd'%3E%3Cg fill='%239C92AC' fill-opacity='0.05'%3E%3Cpath opacity='.5' d='M96 95h4v1h-4v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4h-9v4h-1v-4H0v-1h15v-9H0v-1h15v-9H0v-1h15v-9H0v-1h15v-9H0v-1h15v-9H0v-1h15v-9H0v-1h15v-9H0v-1h15v-9H0v-1h15V0h1v15h9V0h1v15h9V0h1v15h9V0h1v15h9V0h1v15h9V0h1v15h9V0h1v15h9V0h1v15h9V0h1v15h4v1h-4v9h4v1h-4v9h4v1h-4v9h4v1h-4v9h4v1h-4v9h4v1h-4v9h4v1h-4v9h4v1h-4v9zm-1 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-9-10h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm9-10v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-9-10h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm9-10v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-9-10h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm9-10v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-10 0v-9h-9v9h9zm-9-10h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9zm10 0h9v-9h-9v9z'/%3E%3Cpath d='M6 5V0H5v5H0v1h5v94h1V6h94V5H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
}

/* Utility classes */
.text-sm { font-size: var(--font-size-sm); }
.text-base { font-size: var(--font-size-base); }
.text-lg { font-size: var(--font-size-lg); }
.text-xl { font-size: var(--font-size-xl); }

.font-medium { font-weight: 500; }
.font-semibold { font-weight: 600; }
.font-bold { font-weight: 700; }

.rounded-sm { border-radius: var(--border-radius-sm); }
.rounded { border-radius: var(--border-radius-base); }
.rounded-md { border-radius: var(--border-radius-md); }
.rounded-lg { border-radius: var(--border-radius-lg); }

.shadow-sm { box-shadow: var(--shadow-sm); }
.shadow { box-shadow: var(--shadow-base); }
.shadow-md { box-shadow: var(--shadow-md); }
.shadow-lg { box-shadow: var(--shadow-lg); }

.transition { transition: var(--transition-base); }
</style>