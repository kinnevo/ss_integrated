<template>
<v-main fluid>
    <v-layout row wrap>
      <v-flex xs12 class="text-xs-center" mt-5>
        <h1>Moment</h1>
      </v-flex>
      <v-flex xs12 class="text-xs-center" mt-3>
        <p>Define a moment in a video that helps to understand a challenge</p>
      </v-flex>
    </v-layout>

      <v-row>
        Discover the value in this learning experience: {{title}}
      </v-row>

      <v-row  style="height: 500px;">
        <v-col class="pa-8">
                      
          <iframe
              width="100%"
              height="100%"
              :src=url_experiencia
              frameborder="0"
              allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
              allowfullscreen
          ></iframe>
        </v-col>

        <v-col class="pa-4">
        <v-container id="input-usage" >
          <v-row>
            Time: 
            <v-text-field v-model="ss_moment_moment_time">
            Label="Introduce the timemark where you define the moment in the video"
            </v-text-field>
          </v-row>
        </v-container>
          <v-row>
            <v-col
                cols="40"
                md="12"
              >
              <v-textarea
                filled
                v-model="ss_moment_moment"                         

                name="input-7-4"
                label="Moment"
                value=moment
                auto-grow
                rows="15"
                row-height="15"
              ></v-textarea>

            </v-col>
          </v-row>
        </v-col>
      </v-row>
    
      <v-row>
        <v-col class="pa-6">
          <p>URL del video:</p><span>{{url_experiencia}}</span> ---- {{ss_moment_moment}} ---- is here

        </v-col>

        <v-col class="pa-6">  
          <v-row>
            <v-col>
              <v-btn class="pa-2"
                depressed
                color="primary"
                @click=clickea
              >
                Insert
              </v-btn>
            </v-col> 

            <v-col> 
              <v-btn class="pa-2"
                depressed
                color="primary"
              >
                Edit
              </v-btn>
            </v-col>

            <v-col> 
              <v-btn class="pa-2"
                depressed
                color="primary"
              >
                Delete
              </v-btn>
            </v-col> 
          </v-row>
        </v-col>
      </v-row>

      <v-row no-gutters>
          <v-col
            v-for="n in 3"
            :key="n"
            cols="12"
            sm="4"
          >
            <v-card
              class="pl-10 py-5"
              outlined
              tile
            >
              One of three columns
            </v-card>
          </v-col>
      </v-row>
</v-main>
</template>
    
<script>
  export default {
    name: "MomentView",
    data() {

      return {
          moment_view: "",
          title: "Video de pruebas",
          url_experiencia: "https://youtube.com/embed/WCUGI8PGcGw",
          time: 10,
          moment: "cadena de caracteres",
          btn_insert: "Insert",
          btn_edit: "Edit",
          btn_delete: "Delete",
          textarea: null,
          ss_moment_moment: "",
          ss_moment_moment_time: 33,

      }
    },
    methods: {
        clickea(){
          console.log( "Captura: " + this.ss_moment_moment + "----" + this.ss_moment_moment_time );
        }
    },

    computed: {
      messageClass(){
        return {
          'md-invalid': this.hasMessages
        }
      }
    }
  }
  
</script>
