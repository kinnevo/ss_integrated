import Vue from 'vue'
import App from './App.vue'

import vuetify from './plugins/vuetify'
import router from './router'

global.Buffer = global.Buffer || require("buffer").Buffer;

// global variable

/*
import globalConstants from "./globalConstant"
import * as nearAPI from 'near-api-js'
const {keyStores} = nearAPI;

Vue.prototype.$CONTRACT_ID = "dev-1656452729299-85030592138402";
Vue.prototype.$config = {
  networkId: 'testnet',
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org'
};
*/

Vue.config.productionTip = false

new Vue({
  vuetify,
  router,
  render: h => h(App)
}).$mount('#app')
