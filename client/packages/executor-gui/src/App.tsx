import { useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import { ExecutorControlProvider } from "./contexts/ExecutorControlContext";
import { LoggerProvider } from "./contexts/LoggerContext";
import { StoreProvider } from "./contexts/StoreContext";
import { ViewProvider } from "./contexts/ViewContext";
import { AppRoot } from "./components/AppRoot";

export function App() {
  useEffect(() => {
    invoke("init");
  }, []);

  return (
    <ViewProvider>
      <StoreProvider>
        <LoggerProvider>
          <ExecutorControlProvider>
            <AppRoot />
          </ExecutorControlProvider>
        </LoggerProvider>
      </StoreProvider>
    </ViewProvider>
  );
}
