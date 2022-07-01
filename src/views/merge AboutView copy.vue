<template>
  <div class="about">
    <h1>This is a page to test each access to the data in the blockchain</h1>

    <div>
      <p>Screen: {{ experienciesview.length }}</p>
      <span v-for="item in experienciesview.length" :key="item">
        <p class="text-left">item: {{item}}: {{ experienciesview[item-1] }}</p>
        <p class="text-left">item: {{item}}: {{ experienciesview[item-1].description }}</p>

      </span>
    </div>

  </div>
</template>

<script>
 


import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores, Contract } = nearAPI;

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
    name: "AboutView",
    created(){
      this.disp_experiences();
      console.log( "testing")
    },

    data(){
      return {
        video_info: 0,
        exp_list: [""],
        exp_info: "",
        products:[
          {id: 0, name: 'shirt'},
          {id: 1, name: 'jacket'},
          {id: 2, name: 'shoes'},
        ],
        experienciesview: [""],
      }
    },

    methods: {
        async disp_experiences(){
          const near = await connect(config);
          const wallet = new WalletConnection(near, 'ss');

          const contract = new Contract(wallet.account(), CONTRACT_ID, {
            viewMethods:  ['getNumber_of_experiences', 'getUser_exp','getExperience'],
            sender: wallet.account()
          });

          // use a contract view method
          this.video_info = await contract.getNumber_of_experiences();
          console.log( this.video_info );

          this.exp_list = await contract.getUser_exp({
            "wallet": "zavala55.testnet"
          });
          console.log( this.exp_list );

          for ( let i = 0 ; i < this.exp_list.length ; i++  ){

            this.exp_info = await contract.getExperience({
              video_n: this.exp_list[i]
            });
            console.log( this.exp_info );
            this.experienciesview[i] = this.exp_info;
          }
          
          console.log( this.experienciesview );
          console.log( screen );

        }
      
    },
  }
</script>
