import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import { useStore } from 'vuex';

import Home from '../views/Home.vue';
import Login from '../views/Login.vue';
import Admin from '../views/Admin.vue';
import Studio from '../views/Studio.vue';
import Dashboard from '../views/studio/Dashboard.vue';
import Pipelines from '../views/studio/Pipelines.vue';
import Settings from '../views/studio/Settings.vue';
import DockerFiles from '../views/studio/DockerFiles.vue';
import AmberStores from '../views/studio/AmberStores.vue';
import Jobs from '../views/studio/Jobs.vue';
import JobDetail from '@/views/studio/JobDetail.vue'
import JobData from '@/views/studio/JobData.vue'
import JobLogs from '@/views/studio/JobLogs.vue'

const routes: Array<RouteRecordRaw> = [
  { path: '/', name: 'Home', component: Home },
  { path: '/login', name: 'Login', component: Login },
  { path: '/admin', name: 'Admin', component: Admin },
  {
    path: '/studio',
    name: 'Studio',
    component: Studio,
    meta: { requiresAuth: true },
    children: [
      { path: 'dashboard', name: 'Dashboard', component: Dashboard },
      {
        path: 'jobs',
        name: 'Jobs',
        component: Jobs,
      },
      {
        path: 'jobs/:id',
        name: 'JobDetail',
        component: JobDetail,
      },
      {
        path: 'jobs/:id/data',
        name: 'JobData',
        component: JobData,
      },
      {
        path: 'jobs/:id/logs',
        name: 'JobLogs',
        component: JobLogs,
      },
      {
        path: 'pipelines',
        name: 'Pipelines',
        component: Pipelines,
      },
      {
        path: 'pipelines/:id',
        name: 'PipelineEditor',
        component: () => import('@/components/studio/editors/PipelineEditor.vue'),
        props: (route) => ({ id: route.params.id, returnToJobDetails: route.query.returnToJobDetails }),
      },
      { path: 'settings', name: 'Settings', component: Settings },
      {
        path: 'dockerfiles',
        name: 'DockerFiles',
        component: DockerFiles,
      },
      {
        path: 'dockerfiles/new',
        name: 'NewDockerFile',
        component: () => import('@/components/studio/editors/DockerFileEditor.vue'),
      },
      {
        path: 'dockerfiles/:id',
        name: 'DockerFileEditor',
        component: () => import('@/components/studio/editors/DockerFileEditor.vue'),
        props: (route) => ({ id: route.params.id, returnToJobDetails: route.query.returnToJobDetails }),
      },
      {
        path: 'configurations',
        name: 'Configurations',
        component: () => import('@/views/studio/Configurations.vue'),
      },
      {
        path: 'configurations/new',
        name: 'NewConfiguration',
        component: () => import('@/components/studio/editors/ConfigurationEditor.vue'),
        props: { id: null },
      },
      {
        path: 'configurations/:id',
        name: 'ConfigurationEditor',
        component: () => import('@/components/studio/editors/ConfigurationEditor.vue'),
        props: (route) => ({ id: route.params.id, returnToJobDetails: route.query.returnToJobDetails }),
      },
      {
        path: 'amberstores',
        name: 'AmberStores',
        component: AmberStores,
      },
      {
        path: 'amberstore/:id?',
        name: 'AmberStoreEditor',
        component: () => import('@/components/studio/editors/AmberStoreEditor.vue'),
        props: (route) => ({ id: route.params.id, returnToJobDetails: route.query.returnToJobDetails }),
      },
    ],
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Global navigation guard for debugging
router.beforeEach((to, from, next) => {
  console.log(`Navigation from ${from.fullPath} to ${to.fullPath}`);
  next();
});

// Navigation Guard
router.beforeEach((to, _, next) => {
  const store = useStore();
  const isLoggedIn = store.state.isLoggedIn;

  if (to.matched.some(record => record.meta.requiresAuth)) {
    if (!isLoggedIn) {
      next('/login');
    } else {
      next();
    }
  } else if (to.path === '/login' && isLoggedIn) {
    next('/studio/dashboard');
  } else {
    next();
  }
});

export default router;