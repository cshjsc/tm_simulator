<script setup>
import { ref } from "vue";

// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

defineProps({
  msg: String,
});

const count = ref(0);

async function helloFromRust() {
  const executor = await invoke("get_test_machine");
  console.log(executor);
  count.value++;
}

async function executeStep() {
  const executor = await invoke("accept_input", { input: "test" });
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
  </p>

  <p>
    <a href="https://vitejs.dev/guide/features.html" target="_blank">
      Vite Documentation
    </a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Documentation</a>
  </p>

  <button type="button" @click="helloFromRust()">count is: {{ count }}</button>
  <button type="button" @click="executeStep()">execute me</button>
  <p>
    Edit
    <code>components/HelloWorld.vue</code> to test hot module replacement.
  </p>
</template>

<style scoped>
a {
  color: #42b983;
}
</style>
