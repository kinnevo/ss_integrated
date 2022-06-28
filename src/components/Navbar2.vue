<template>
  <nav>

    <v-toolbar flat app>
      <v-toolbar-title class="text-uppercase grey--text">
        <span class="font-weight-light">Sharing</span>
        <span>Shard</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
        <!-- added menu -->
        <v-btn flat color="green" router :to="commands[0].route">
        <!-- <v-btn flat color="green" :href="commands[0].route"> -->

          <span>{{ commands[0].text }}</span>
        </v-btn>

        <v-btn flat color="blue" router :to="commands[1].route">
        <!-- <v-btn flat color="green" :href="commands[1].route"> -->

          <span>{{ commands[1].text }}</span>
        </v-btn>




      <v-btn flat color="grey">
        <span>Sign Out</span>
        <v-icon right>exit_to_app</v-icon>
      </v-btn>
    </v-toolbar>



  </nav>
</template>

<script>
export default {
    name: 'Navbar',
  data() {
    return {
      drawer: false,
      links: [
        { icon: 'dashboard', text: 'Dashboard', route: '/' },
        { icon: 'folder', text: 'My Projects', route: '/projects' },
        { icon: 'person', text: 'Team', route: '/team' },
      ],
      commands: [
        { text: 'Experiences', route: '/experiences'},
        { text: 'Points of View', route: '/pov'},
      ]
    }
  }
}
</script>

<style>

</style>
