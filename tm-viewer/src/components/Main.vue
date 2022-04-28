<script setup>
import { ref } from "vue";

// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
import Machine from "./Machine.vue";
const machine = ref(undefined);
const steps = ref([]);

async function getTestMachine() {
  machine.value = await invoke("get_test_machine");
}
async function accept_input() {
  steps.value = await invoke("accept_input", { input: ["0", "1", "0", "0"] });
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
