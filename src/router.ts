import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior(to, from, savedPosition) {
    // Se l'utente usa back/forward → torna alla posizione salvata
    if (savedPosition) {
      return savedPosition;
    }

    // Se è una navigazione normale tra pagine → vai in alto
    return { top: 0 };
  },
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
      path: "/auth/confirmed",
      name: "auth_confirmed",
      component: () => import("./views/AuthConfirmed.vue")
    },
    {
      path: "/devpipeline",
      name: "pipeline",
      component: () => import("./views/DevPipeline.vue")
    },
    {
      path: "/:pathMatch(.*)*",
      name: "error",
      component: () => import("./views/Error.vue"),
    }
  ],
});

export default router;
