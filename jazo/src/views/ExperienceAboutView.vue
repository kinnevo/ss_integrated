<template>
  <div class="about">
    <h1>This is a page to test each access to the data in the blockchain</h1>
    <h2>Read the experiences created by a user</h2>
    <div class="text-left">
    <code>
        Read the number of experiences of a given user<br/>
        Read the information of each experience
        Build the structure to display information

experienciesview: [
        { 
            title: "Aprendizajes en el Silicon Valley",
            owner: "zavala55.testnet",
            description: "Narración de la Experiencia de Virgilio Raiden en su visita a SV", 
            url: "https://youtube.com/embed/WCUGI8PGcGw",
            topic: 8,
            reward: 11,
            exp_date: 22,
            moment: "TBD",
            time: 33,
            pov: "TBD",
            status: "En proceso",
         },

near view dev-1656452729299-85030592138402 getNumber_of_experiences --accountId zavala55.testnet

    </code>
    </div>
    <p>Display the information received</p>
    The user:  has created {{this.video_info}} experiences
    <!--
      <span v-for="(item,index) in this.exp_list">
            <p>{{ index}}</p>
      </span>
<span v-for="exp in exp_list">
<li  {{ exp }} </li>
</span>

<ul v-if="exp_list.length" :key>
  <li v-for="exp in exp_list">
    {{ exp }}
  </li>
</ul>
-->
<div>
  <p>exp_list: {{ exp_list.length }}</p>
  <span v-for="exp in exp_list.length" :key="exp">
    {{ exp }}
  </span>
  <p>Experiencia: {{ exp_info.description }}</p>
  <p>URL: {{ exp_info.url }}</p>
</div>


<ul>
  <li v-for="product in products"      :key='product.id' >
    {{ product.name }}
  </li>
</ul>

<div>
  <p>Screen: {{ x_screen_data.length }}</p>
  <span v-for="item in x_screen_data.length" :key="item">
    <p class="text-left">item: {{item}}: {{ x_screen_data[item-1] }}</p>
</span>
</div>

<div>
  <p>Screen: {{ x_screen_data.length }}</p>
  <span v-for="item in x_screen_data.length" :key="item">
    <p class="text-left">item: {{item}}: {{ x_screen_data[item-1].description }}</p>
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
