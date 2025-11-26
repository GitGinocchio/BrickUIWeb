import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("./views/HomeView.vue"),
      beforeEnter: (to, from, next) => {
        if (to.query.status_code && from.name !== 'error') {
          return next({
            name: 'error',
            query: to.query
          })
        }
        next()
      }
    },
    {
      path: "/about",
      name: "about",
      component: () => import("./views/AboutView.vue"),
    },
    {
      path: "/:pathMatch(.*)*",
      name: "error",
      component: () => import("./views/Error.vue"),
    }
  ],
});

export default router;
