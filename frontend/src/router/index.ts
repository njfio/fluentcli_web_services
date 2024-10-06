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
        path: '/studio/jobs',
        name: 'Jobs',
        component: Jobs,
        meta: { requiresAuth: true }
      },
      {
        path: '/studio/jobs/:id',
        name: 'JobDetail',
        component: JobDetail
      },
      {
        path: '/studio/jobs/:id/data',
        name: 'JobData',
        component: JobData
      },
      {
        path: '/studio/jobs/:id/logs',
        name: 'JobLogs',
        component: JobLogs
      },
      {
        path: 'pipelines/:id?',
        name: 'PipelineEditor',
        component: Pipelines,
        props: (route) => ({ returnToJobDetails: route.query.returnToJobDetails === 'true' }),
        meta: { requiresAuth: true }
      },
      { path: 'settings', name: 'Settings', component: Settings },
      {
        path: 'dockerfiles',
        name: 'DockerFiles',
        component: DockerFiles,
        meta: { requiresAuth: true }
      },
      {
        path: '/studio/configurations',
        name: 'Configurations',
        component: () => import('@/views/studio/Configurations.vue'),
        meta: { requiresAuth: true }
      },
      {
        path: '/studio/configurations',
        name: 'ConfigurationsList',
        component: () => import('@/views/studio/Configurations.vue'),
        meta: { requiresAuth: true }
      },
      {
        path: '/studio/configurations/:id?',
        name: 'ConfigurationEditor',
        component: () => import('@/views/studio/Configurations.vue'),
        meta: { requiresAuth: true }
      },
      {
        path: '/configurations/:id?',
        name: 'Configurations',
        component: () => import('@/views/studio/Configurations.vue')
      },
      {
        path: '/studio/amberstores',
        name: 'AmberStores',
        component: AmberStores,
        meta: { requiresAuth: true }
      },

    ],
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
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