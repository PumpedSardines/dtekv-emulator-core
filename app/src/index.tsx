import * as React from "react";
import * as ReactDOM from "react-dom/client";

import './global.scss';
import HexDisplay from "./components/HexDisplay";
import Emulator from "./pages/Emulator";


const App = () => {
  return (
    <Emulator />
  );
};

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(<App />);
