<template>
  <v-container>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>Empire Data Input</v-card-title>

          <v-card-text>
            <v-text-field
              :value="empire_data.countdown"
              @change="onCountdownChange"
              type="number"
              label="Countdown"
            />

            <span>Bounty hunters</span>
            <v-btn icon @click="addBountyHunter"><v-icon>mdi-plus</v-icon></v-btn>
            <v-list>
              <v-list-item v-for="bh in empire_data.bounty_hunters" :key="bh.day">
                <v-list-item-content>
                  <v-container>
                    <v-row>
                      <v-col cols="5">
                        <v-text-field
                            v-model="bh.planet"
                            label="Planet"
                        />
                      </v-col>
                      <v-col cols="5">
                        <v-text-field
                            v-model="bh.day"
                            type="number"
                            label="Day"
                        />
                      </v-col>
                      <v-col cols="2">
                        <v-btn text icon color="red"><v-icon>mdi-delete</v-icon></v-btn>
                      </v-col>
                    </v-row>
                  </v-container>
                </v-list-item-content>
              </v-list-item>
            </v-list>
          </v-card-text>

          <v-card-actions>
                <v-file-input
                    v-model="file"
                    accept=".json"
                    truncate-length="15"
                    label="Upload stolen data"
                    style="max-width: 200px"
                    @change="onFileUpload"
                ></v-file-input>

            <v-btn text x-large @click="simulate" color="red">Simulate</v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import EmpireData from "@/models/empire_data";

@Component
export default class SettingsCard extends Vue {
  file = null;

  get empire_data(): EmpireData {
    if (this.$store)
      return this.$store.getters["app/getEmpireData"];
    else
      return {countdown: 0, bounty_hunters: []}
  }

  onCountdownChange(value: number) {
    // This '+' is a small hack fix... Vuetify sends "value" as a string, despite the number type.
    this.$store.dispatch("app/setCountdown", +value);
  }

  onFileUpload(value: File) {
    const reader = new FileReader();
    reader.onload = (e: any) => {
      let empire_data = JSON.parse(e.target.result) as EmpireData;
      this.$store.commit("app/setEmpireData", empire_data);
      this.file = null;
    }

    reader.readAsText(value);
  }

  addBountyHunter() {
    this.$store.dispatch("app/addBountyHunter", {planet: "", day: 0})
  }

  simulate() {
    this.$store.dispatch("app/simulate");
  }
}
</script>

<style scoped></style>
