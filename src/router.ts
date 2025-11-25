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
            query: { status_code: to.query.status_code }
          })
        }
        next()
      }
    },
    {
      path: "/about",
      name: "about",
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
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
