<template>
  <v-app>
    <v-main>
      <v-container>
        <v-row>
          <v-col>
            <ResultCard />
          </v-col>
        </v-row>
        <v-row>
          <v-col>
            <SettingsCard />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import ResultCard from "@/components/ResultCard.vue";
import SettingsCard from "@/components/SettingsCard.vue";

export default Vue.extend({
  name: "App",

  components: {
    ResultCard,
    SettingsCard,
  },

  data: () => ({
    //
  }),
});
</script>
