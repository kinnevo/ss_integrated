<template>
  <v-toolbar text>
      <v-toolbar-title class="text-uppercase grey--text">
        <span class="font-weight-light">Sharing</span>
        <span class="font-weight-black">Shard</span>
      </v-toolbar-title>

      <v-spacer></v-spacer>

      <span v-for="command in commands" :key="command.text" >
            <v-btn router :to="command.route" text :color="command.color" >{{ command.text }}</v-btn>
      </span>

      <v-btn text color="grey" @click=login_action>
        <span>{{ this.login_status }}</span>
        <v-icon>mdi-logout-variant</v-icon>
      </v-btn>
  </v-toolbar>
</template>

<script>
import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores } = nearAPI;

const CONTRACT_ID = "dev-1656452729299-85030592138402";
const config = {
  networkId: 'testnet',
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org'
};


export default {
    name: 'NavBar',
  data() {
    return {
      drawer: false,
      login_status: "Logout",
      user_logged: "",
      commands: [
        { color: "green", text: 'Experiences', route: '/experiences'},
        { color: "blue", text: 'Moment', route: '/moment'},
        { color: "red", text: 'Points of View', route: '/pov'},
        { color: "brown", text: 'Test', route: '/about'},
      ]
    }
  },
  created(){
    this.is_Logged();
  },

  methods: {
    async login_action() {
        const near = await connect(config);
        const wallet = new WalletConnection(near);

        if (this.login_status == "Login") {
                    wallet.requestSignIn( CONTRACT_ID, 'sharingshard');
          this.user_logged = wallet.getAccountId();
          this.login_status = this.user_logged;  //"Logout"
          console.log( "Logout");
        }else {
          wallet.signOut();
          this.login_status = "Login"
          console.log( "Login " ); 
        }
    },
    async is_Logged() {
//        const { connect, keyStores, WalletConnection } = nearAPI;

        const near = await connect(config);
        const wallet = new WalletConnection(near);

        if (wallet.isSignedIn()) {
          this.user_logged = wallet.getAccountId();
          this.login_status = this.user_logged
//          this.login_status = "Logout"
//          alert( "SignedIN:" + this.user_logged);
        }
    }

  }
}
</script>

<style>

</style>
