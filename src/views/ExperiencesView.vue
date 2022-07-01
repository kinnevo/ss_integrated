<template>
<div class="c_experienciesview">
    <h1 class="subheading grey--text">Experiences</h1>
    <p>Choose the experiences where you want to collaborate</p>
    <v-container class="my-5">
      <v-layout row wrap>
<!--      <p>Number of  of Experiences: {{ experienciesview.length }}</p> -->
<!--      <p>Number of  of Experiences: {{ experienciesview }}</p><br/> -->
      <p>Number of  of Experiences: {{ exp_info }}</p>



        <v-flex xs12 sm6 md4 lg3 v-for="item in experienciesview.length" :key="item">
          <v-card text class="text-xs-center ma-3">
            <v-responsive class="pt-3">
              <iframe
                  width="100%"
                  height="100%"
                  :src=experienciesview[item-1].url
                  frameborder="0"
                  allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
                  allowfullscreen
              ></iframe>
            </v-responsive>
          <v-card-text>
            <div class="subheading text-sm-left">{{ experienciesview[item-1].title }}</div>

            <div class="subheading text-sm-left">{{ experienciesview[item-1].description }}</div>
            <div class="grey--text"><p class="text-sm-left">Reward: {{ experienciesview[item-1].reward }}</p></div>
            <div class="subheading text-sm-left">By: {{ experienciesview[item-1].owner }}</div>

            <v-divider></v-divider>

            <div class="grey--text">{{ experienciesview[item-1].url }}</div>
          </v-card-text>
          <v-card-actions>
            <v-btn text color="grey">
              <v-icon small left>mdi-message</v-icon>
              <span class="">Moment</span>
            </v-btn>
          </v-card-actions>

          <v-card-actions>
            <v-btn text color="grey">
              <v-icon small left>mdi-message</v-icon>
              <span class="">PoV</span>
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-flex>




    </v-layout>
  </v-container>
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
  name: "ExperiencesView",
  /* https://learnvue.co/tutorials/vue-lifecycle-hooks-guide
    created()
    mounted()
    updated()
  */
  created(){
    this.disp_experiences();
  },  

/*
struct Experience{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    topic: u8,
    reward: Balance,
    exp_date: i64,
    moment: String,
    time: u16,
    pov: UnorderedMap<AccountId, String>,
    status: String,
}

"wallet": "zavala55.testnet",

"experience_name": "Aprendizajes en el Silicon Valley", 
"description": "Narración de la Experiencia de Virgilio Raiden en su visita a SV", 
"url": "https://youtube.com/embed/WCUGI8PGcGw",
"reward": 10, 
"expire_date":20, 
"topic":8
*/

  data() {
    return {
      video_info: 0,
      exp_list: [""],
      exp_info: "",
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
    }


  },
}
</script>
