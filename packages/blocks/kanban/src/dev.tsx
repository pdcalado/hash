/**
 * This is the entry point for developing and debugging.
 * This file is not bundled with the block during the build process.
 */
import { MockBlockDock } from "mock-block-dock";
import { render } from "react-dom";

import exampleGraph from "../example-graph.json";
import Component from "./index";

const node = document.getElementById("app");

const DevApp = () => {
  return (
    <MockBlockDock
      blockDefinition={{ ReactComponent: Component }}
      blockEntity={{
        entityId: "kanban1",
        properties: { name: "World" },
      }}
      initialEntities={exampleGraph.entities}
      initialLinks={exampleGraph.links}
      debug
    />
  );
};

render(<DevApp />, node);
