import Vue from 'vue'
import Router from 'vue-router'
import PoView from './views/PoView.vue'
//import HomeView from './views/HomeView.vue'
import MomentView from './views/MomentView.vue'
import ExperiencesView from './views/ExperiencesView.vue'
import AboutView from './views/AboutView.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/moment',
      name: 'moment',
      component: MomentView
    },
    {
      path: '/experiences',
      name: 'experiencesview',
      component: ExperiencesView
    },
    {
      path: '/pov',
      name: 'pov',
      component: PoView
    },
    {
      path: '/about',
      name: 'about',
      component: AboutView
    }
  ]
})
