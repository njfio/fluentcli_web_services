import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import Login from '../views/Login.vue';
import Admin from '../views/Admin.vue';
import Studio from '../views/Studio.vue';
import Dashboard from '../views/studio/Dashboard.vue';
import Jobs from '../views/studio/Jobs.vue';
import Pipelines from '../views/studio/Pipelines.vue';
import Settings from '../views/studio/Settings.vue';
import JobList from '../views/admin/JobList.vue';
import JobDetails from '../views/admin/JobDetails.vue';
import JobCreate from '../views/admin/JobCreate.vue';
import JobEdit from '../views/admin/JobEdit.vue';
import WorkerTypeList from '../views/WorkerTypeList.vue'; // This one remains in the root views folder
import store from '@/store';

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
      { path: 'jobs', name: 'Jobs', component: Jobs },
      { path: 'pipelines', name: 'Pipelines', component: Pipelines },
      { path: 'settings', name: 'Settings', component: Settings },
    ],
  },
  { path: '/admin/jobs', name: 'JobList', component: JobList },
  { path: '/admin/jobs/create', name: 'JobCreate', component: JobCreate },
  { path: '/admin/jobs/:id', name: 'JobDetails', component: JobDetails },
  { path: '/admin/jobs/:id/edit', name: 'JobEdit', component: JobEdit },
  { path: '/worker-types', name: 'WorkerTypeList', component: WorkerTypeList },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Navigation Guard
router.beforeEach((to, _, next) => {
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth);
  const isLoggedIn = store.getters.isLoggedIn;

  if (requiresAuth && !isLoggedIn) {
    next('/login');
  } else if (to.path === '/login' && isLoggedIn) {
    next('/');
  } else {
    next();
  }
});

export default router;