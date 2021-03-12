<template>
  <v-card elevation="2" class="mx-auto my-auto" dark>
    <v-card-title>Signin</v-card-title>
    <v-card-text>
      <form @submit.prevent="submit">
        <v-text-field v-model="email" label="E-mail" required></v-text-field>
        <v-text-field
          v-model="password"
          label="Password"
          type="password"
          required
        ></v-text-field>
      </form>
    </v-card-text>
    <v-card-actions align="right">
      <v-btn type="submit" text @click="onSubmit"> submit </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({
  data: () => ({
    email: "",
    password: "",
  }),
  methods: {
    onSubmit(): void {
      console.log(this.email);
    },
  },
});
</script>