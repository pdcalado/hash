/**
 * This is the entry point for developing and debugging.
 * This file is not bundled with the library during the build process.
 */
import { MockBlockDock } from "mock-block-dock";
import { render } from "react-dom";

import Component from "./index";

const node = document.getElementById("app");

const App = () => (
  <MockBlockDock>
    <Component color="red" entityId="divider1" height="2px" />
  </MockBlockDock>
);

render(<App />, node);
