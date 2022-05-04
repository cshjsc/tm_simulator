<script setup>
import { ref } from "vue";

// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
import Machine from "./Machine.vue";
const machine = ref(undefined);
const steps = ref([]);

async function getTestMachine() {
  //machine.value = await invoke("get_test_machine");
  machine.value = {
    alphabet: ["0", "_", "1"],
    size: 1,
    start_state_name: "q0",
    states: {
      q0: {
        is_end_state: false,
        transition_functions: [
          {
            bands_actions: [["1", "Right"]],
            bands_requirements: ["0"],
            next_state_name: "q0",
            origin: "q0",
          },
          {
            bands_actions: [["1", "Right"]],
            bands_requirements: ["1"],
            next_state_name: "q0",
            origin: "q0",
          },
          {
            bands_actions: [["_", "Unchanged"]],
            bands_requirements: ["_"],
            next_state_name: "q1",
            origin: "q0",
          },
        ],
      },
      q1: { is_end_state: true, transition_functions: [] },
    },
  };
}
async function accept_input() {
  //steps.value = await invoke("accept_input", { input: ["0", "1", "0", "0"] });
}
</script>

<template>
  <h1>TM Machine Simulator</h1>
  <div v-if="machine !== undefined">
    <Machine :machine="machine"></Machine>
    <button @click="accept_input()">start machine</button>
    <p v-for="step in steps">step: {{ step }}</p>
  </div>
  <div v-else>
    <button type="button" @click="getTestMachine()">get machine</button>
    <p>select a machine to contnue</p>
  </div>
</template>

<style scoped>
a {
  color: #42b983;
}
</style>
