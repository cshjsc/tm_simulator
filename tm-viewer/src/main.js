import { createApp } from "vue";
import App from "./App.vue";
import VNetworkGraph from "v-network-graph";

const app = createApp(App);
app.use(VNetworkGraph);
app.mount("#app");
