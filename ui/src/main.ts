import './component-hook';
import Vue from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import { Store } from 'vuex';

Vue.config.productionTip = false;

new Vue({
  router,
  store: store as Store<any>,
  render: (h) => h(App),
}).$mount('#app');
