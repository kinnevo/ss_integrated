import Vue from 'vue'
import Router from 'vue-router'
import Pov from './views/Pov.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/pov',
      name: 'pov',
      component: Pov
    }
  ]
})
