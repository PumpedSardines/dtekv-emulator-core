import * as React from "react";
import * as ReactDOM from "react-dom/client";

import './global.scss';
import HexDisplay from "./components/HexDisplay";


const App = () => {
  return (
    <div>
      <HexDisplay value={0x37} />
      <h1>Hello, DTEKV!</h1>
    </div>
  );
};

const root = ReactDOM.createRoot(document.getElementById("root")!);
root.render(<App />);
