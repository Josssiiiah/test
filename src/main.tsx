import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import TestWindow from "./components/testWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";

const Main = () => {
  const [windowType, setWindowType] = useState<string>("main");

  useEffect(() => {
    const checkWindowType = async () => {
      try {
        const win = getCurrentWindow();
        const label = await win.label;
        if (label === "test") {
          setWindowType("test");
        } else {
          setWindowType("main");
        }
      } catch (error) {
        console.error("Failed to get window label:", error);
      }
    };

    checkWindowType();
  }, []);

  // Render the appropriate component based on window type
  switch (windowType) {
    case "test":
      return <TestWindow />;
    default:
      return <App />;
  }
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>
);
