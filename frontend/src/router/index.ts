import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import Admin from '../views/Admin.vue';
import Studio from '../views/Studio.vue';

const routes: Array<RouteRecordRaw> = [
  { path: '/', name: 'Home', component: Home },
  { path: '/admin', name: 'Admin', component: Admin },
  { path: '/studio', name: 'Studio', component: Studio },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;