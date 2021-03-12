<template>
  <tr>
    <td>{{ $props.name }}</td>
    <td>{{ $data.show ? $props.password : "*************" }}</td>
    <td>
      <v-row>
        <v-spacer></v-spacer>
        <v-btn color="green darken-4" @click="onClick">Show</v-btn>
        <v-spacer></v-spacer>
        <v-btn color="red darken-4">Delete</v-btn>
        <v-spacer></v-spacer>
      </v-row>
    </td>
  </tr>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  data: () => ({
    show: false,
  }),
  methods: {
    onClick(): void {
      this.show = !this.show;
    },
  },
  props: {
    name: String,
    password: String,
  },
});
</script>