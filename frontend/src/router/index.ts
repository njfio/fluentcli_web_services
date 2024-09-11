import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import Login from '../views/Login.vue';
import Admin from '../views/Admin.vue';
import Studio from '../views/Studio.vue';
import JobList from '../views/admin/JobList.vue';
import JobDetails from '../views/admin/JobDetails.vue';
import JobCreate from '../views/admin/JobCreate.vue';
import JobEdit from '../views/admin/JobEdit.vue';
import WorkerTypeList from '../views/WorkerTypeList.vue'; // This one remains in the root views folder

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/login',
    name: 'Login',
    component: Login
  },
  {
    path: '/admin',
    name: 'Admin',
    component: Admin,
    meta: { requiresAuth: true }
  },
  {
    path: '/studio',
    name: 'Studio',
    component: Studio,
    meta: { requiresAuth: true }
  },
  { path: '/admin/jobs', name: 'JobList', component: JobList },
  { path: '/admin/jobs/:id', name: 'JobDetails', component: JobDetails },
  { path: '/admin/jobs/create', name: 'JobCreate', component: JobCreate },
  { path: '/admin/jobs/:id/edit', name: 'JobEdit', component: JobEdit },
  { path: '/admin/worker-types', name: 'WorkerTypeList', component: WorkerTypeList },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

router.beforeEach((to, _, next) => {
  const isAuthenticated = !!localStorage.getItem('token');
  if (to.matched.some(record => record.meta.requiresAuth) && !isAuthenticated) {
    next('/login');
  } else {
    next();
  }
});

export default router;