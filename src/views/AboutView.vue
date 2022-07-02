<template>
  <div class="about">
    <h1>Create to sides:</h1>
    <ul>
      <li>The button that prepare info to connect to the page that display the moment</li>
      <li>The page that display the moment</li>
      <li></li>
    </ul>

    <div class="text-left">
    <code>
        the button side<br/>
        url
        title
        description
        id
        owner
        reward

        time
        moment
    </code>
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
        x_screen_data: [""],
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
            this.x_screen_data[i] = this.exp_info;
          }
          
          console.log( this.x_screen_data );
          console.log( screen );

        }
      
    },
  }
</script>
