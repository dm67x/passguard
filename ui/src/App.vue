<template>
  <v-app>
    <v-main>
      <v-container fill-height>
        <v-row align="center" justify="center">
          <v-col v-if="$data.session">
            <PasswordTable
              v-bind:passwords="[
                { id: 'google', name: 'Google', password: 'test' },
                { id: 'youtube', name: 'Youtube', password: 'test' },
                { id: 'youtube1', name: 'Youtube', password: 'test' },
                { id: 'youtube2', name: 'Youtube', password: 'test' },
                { id: 'youtube3', name: 'Youtube', password: 'test' },
                { id: 'youtube4', name: 'Youtube', password: 'test' },
                { id: 'youtube5', name: 'Youtube', password: 'test' },
                { id: 'youtube6', name: 'Youtube', password: 'test' },
                { id: 'youtube7', name: 'Youtube', password: 'test' },
                { id: 'youtube8', name: 'Youtube', password: 'test' },
                { id: 'youtube9', name: 'Youtube', password: 'test' },
                { id: 'youtube10', name: 'Youtube', password: 'test' },
                { id: 'youtube11', name: 'Youtube', password: 'test' },
                { id: 'youtube12', name: 'Youtube', password: 'test' },
                { id: 'google1', name: 'Google', password: 'test' },
                { id: 'google2', name: 'Google', password: 'test' },
                { id: 'google3', name: 'Google', password: 'test' },
                { id: 'google4', name: 'Google', password: 'test' },
                { id: 'google5', name: 'Google', password: 'test' },
                { id: 'google6', name: 'Google', password: 'test' },
                { id: 'google7', name: 'Google', password: 'test' },
                { id: 'google8', name: 'Google', password: 'test' },
                { id: 'google9', name: 'Google', password: 'test' },
                { id: 'google10', name: 'Google', password: 'test' },
              ]"
            />
          </v-col>

          <v-col v-else>
            <Signin />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import PasswordTable from "./components/PasswordTable.vue";
import Signin from "./components/Signin.vue";

export default Vue.extend({
  name: "App",
  components: {
    PasswordTable,
    Signin,
  },
  data: () => ({
    session: "",
  }),
});
</script>

<style scoped>
.add-btn {
  top: 50px;
}
</style>
