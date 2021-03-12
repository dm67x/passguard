<template>
  <v-container>
    <v-row align="center" justify="space-around">
      <h1>Passwords</h1>
      <v-btn color="primary" elevation="2" left rounded small
        ><v-icon>mdi-plus</v-icon></v-btn
      >
    </v-row>
    <br />
    <v-simple-table dark>
      <template v-slot:default>
        <thead>
          <tr>
            <th class="text-left">Name</th>
            <th class="text-left">Password</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <password-row
            v-for="password in $props.passwords"
            :key="password.id"
            :name="password.name"
            :password="password.password"
          ></password-row>
        </tbody>
      </template>
    </v-simple-table>
  </v-container>
</template>

<script lang="ts">
import { PropType } from "vue";
import { Component, Vue } from "vue-property-decorator";
import PasswordRow from "./PasswordRow.vue";

interface PasswordSite {
  id: string;
  name: string;
  password: string;
}

@Component({
  props: {
    passwords: {
      type: Array as PropType<Array<PasswordSite>>,
      required: true,
    },
  },
  components: {
    PasswordRow,
  },
})
export default class PasswordTable extends Vue {}
</script>