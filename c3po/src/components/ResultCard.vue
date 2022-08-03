<template>
  <v-container>
    <v-row>
      <v-col cols="2">
        <v-img src="../assets/c3po.png" width="300" />
      </v-col>
      <v-col cols="4">
        <v-card elevation="14" shaped>
          <v-card-title
            >With this data, we have a {{ sim_result }}% chance of making it,
            Han!</v-card-title
          >
        </v-card></v-col
      >
      <v-spacer />
    </v-row>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component
export default class ResultCard extends Vue {
  get sim_result(): number {
    if (this.$store)
      return this.$store.getters["app/getSimulationResult"];
    else
      return 0;
  }
}
</script>

<style scoped></style>
