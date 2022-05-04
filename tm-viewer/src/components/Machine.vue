<script setup>
import * as vNG from "v-network-graph";
import { ForceLayout } from "v-network-graph/lib/force-layout";
const props = defineProps({ machine: Object });
function getGraphFromMachine() {
  let nodes = {};
  let edges = {};
  for (const [name, state] of Object.entries(props.machine.states)) {
    let count = 0;
    let selfTransitions = "";
    state.transition_functions.forEach((transFunction) => {
      if (transFunction.next_state_name === name) {
        selfTransitions = `${selfTransitions} \n ${transFunction.bands_requirements} : ${transFunction.bands_actions}`;
      } else {
        edges[`edge${count++}`] = {
          source: name,
          target: transFunction.next_state_name,
          label: `${transFunction.bands_requirements} : ${transFunction.bands_actions}`,
        };
      }
    });
    let label = selfTransitions.length == 0 ? name : selfTransitions;
    nodes[name] = { name: label, color: "black" };
  }
  return { nodes, edges };
}
const { nodes, edges } = getGraphFromMachine();
const configs = vNG.defineConfigs({
  view: {
    layoutHandler: new ForceLayout({
      positionFixedByDrag: false,
      // * The following are the default parameters for the simulation.
      // * You can customize it by uncommenting below.
      createSimulation: (d3, nodes, edges) => {
        const forceLink = d3.forceLink(edges).id((d) => d.id);
        return d3
          .forceSimulation(nodes)
          .force("edge", forceLink.distance(200))
          .force("charge", d3.forceManyBody())
          .force("collide", d3.forceCollide(50).strength(0.2))
          .force("center", d3.forceCenter().strength(0.05))
          .alphaMin(0.001);
      },
    }),
  },
  node: {
    normal: {
      type: "circle",
      //radius: node => node.size, // Use the value of each node object
      color: (node) => node.color,
    },
    //hover: {
    //  radius: node => node.size + 2,
    //  color: node => node.color,
    //},
    //selectable: true,
    label: {
      visible: true,
    },
    //focusring: {
    //  color: "darkgray",
    //},
  },

  edge: {
    label: {
      fontFamily: undefined,
      fontSize: 11,
      lineHeight: 1.1,
      color: "#000000",
      margin: 4,
      background: {
        visible: true,
        color: "#ffffff",
        padding: {
          vertical: 1,
          horizontal: 4,
        },
        borderRadius: 2,
      },
    },
    normal: {
      width: 1,
      color: "black",
    },
    hover: {
      width: 2,
      color: "#0000aa",
    },
    gap: 20,
    type: "curve",
    margin: 6,
    marker: {
      target: {
        type: "arrow",
        width: 8,
        height: 8,
      },
    },
  },
});
console.log(nodes);
console.log(edges);
const options = {
  physics: {
    enabled: false,
  },
};
</script>
<template>
  <v-network-graph :nodes="nodes" :edges="edges" :configs="configs">
    <template
      #override-node-label="{
        nodeId,
        scale,
        text,
        x,
        y,
        config,
        textAnchor,
        dominantBaseline,
      }"
    >
      <text
        x="0"
        y="0"
        :font-size="9 * scale"
        text-anchor="middle"
        dominant-baseline="central"
        fill="#ffffff"
        >{{ nodeId }}</text
      >

      <text
        v-for="(textt, index) in text.split('\n')"
        :x="x"
        :y="y + index * 10"
        :font-size="config.fontSize * scale"
        :text-anchor="textAnchor"
        :dominant-baseline="dominantBaseline"
        :fill="config.color"
        >{{ textt }}</text
      >
    </template>
    <template #edge-label="{ edge, ...slotProps }">
      <v-edge-label
        :text="edge.label"
        align="center"
        vertical-align="above"
        v-bind="slotProps"
      />
    </template>
  </v-network-graph>

  <p>current machine: {{ machine }}</p>
</template>

<style scoped></style>
