<template>
<div class="experienciesview">
    <h1 class="subheading grey--text">Experiences</h1>
    <p>Choose the experiences where you want to collaborate</p>
    <v-container class="my-5">
      <p>The creation of a component to show the full information about an experience</p>
      <v-layout row wrap>
        <v-flex xs12 sm6 md4 lg3 v-for="exp in experienciesview" :key="exp.title">
          <v-card flat class="text-xs-center ma-3">
            <v-responsive class="pt-3">
              <iframe
                  width="100%"
                  height="100%"
                  :src=exp.url
                  frameborder="0"
                  allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
                  allowfullscreen
              ></iframe>
            </v-responsive>
          <v-card-text>
            <div class="subheading text-sm-left">{{ exp.title }}</div>

            <div class="subheading text-sm-left">{{ exp.description }}</div>
            <div class="grey--text"><p class="text-sm-left">Reward: {{ exp.reward }}</p></div>
            <div class="subheading text-sm-left">By: {{ exp.owner }}</div>

            <v-divider></v-divider>

            <div class="grey--text">{{ exp.url }}</div>
          </v-card-text>
          <v-card-actions>
            <v-btn flat color="grey">
              <v-icon small left>mdi-message</v-icon>
              <span class="">Moment</span>
            </v-btn>
          </v-card-actions>

          <v-card-actions>
            <v-btn flat color="grey">
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
/*
How to test the video display in an iframe
https://codepen.io/sharon-the-encoder/pen/rNWrQJK?editors=1111
*/
export default {
  name: "ExperiencesView",

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
        { 
            title: "Otros aprendizajes",
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
        { 
            title: "Viviendo en el Silicon Valley",
            owner: "zavala55.testnet",
            description: "Narración de la Experiencia de Virgilio Raiden en su visita a SV", 
            url: "https://youtube.com/embed/8IEezGQwgZk",
            topic: 8,
            reward: 11,
            exp_date: 22,
            moment: "TBD",
            time: 33,
            pov: "TBD",
            status: "En proceso",
         },
        { 
            title: "Recuerda el futuro",
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

      ],
      youtubeURL: "https://youtu.be/WCUGI8PGcGw",
      result: "https://youtu.be/WCUGI8PGcGw"
    }
  },
  methods: {

    loadURL () {
      const youtubeEmbedTemplate = 'https://www.youtube.com/embed/'
      const url = this.youtubeURL.split(/(vi\/|v%3D|v=|\/v\/|youtu\.be\/|\/embed\/)/)
      console.log("url", url)
      const YId = undefined !== url[2] ? url[2].split(/[^0-9a-z_/\\-]/i)[0] : url[0]
      console.log("YId", YId)
      if (YId === url[0]) {
        console.log("not a youtube link")
      } else {
        console.log("it's  a youtube video")
      }
      const topOfQueue = youtubeEmbedTemplate.concat(YId)
      console.log("topOfQueue", topOfQueue)
      this.result = topOfQueue
      this.youtubeURL = ''
    }



  }
}
</script>
