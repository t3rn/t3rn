import { ReactNode, createContext, useContext, useRef, useState } from "react";
import { Child, Command } from "@tauri-apps/api/shell";
import { useLogger } from "./LoggerContext";
import { useStore } from "./StoreContext";
import { CIRCUIT_SIGNER_KEY } from "@/consts";
import { getEnv } from "@/utils/config";
import { invoke } from "@tauri-apps/api";

enum Errors {
  NO_CIRCUIT_SIGNER_KEY = "Unable to start the executor! Please configure the circuit signer key",
}

export enum RunningState {
  Idle,
  Active,
}

interface Context {
  state: RunningState;
  start: () => Promise<void>;
  stop: () => Promise<void>;
  restart: () => Promise<void>;
}

const ExecutorControlContext = createContext({} as Context);

interface Props {
  children: ReactNode;
}

export const ExecutorControlProvider = ({ children }: Props) => {
  const store = useStore();
  const { log } = useLogger();
  const [state, setState] = useState(RunningState.Idle);
  const commandRef = useRef<Command | undefined>();
  const childProcessRef = useRef<Child | undefined>();

  const start = async () => {
    const isCircuitSignerKeySet = Boolean(await store.get(CIRCUIT_SIGNER_KEY));
    if (!isCircuitSignerKeySet) throw Errors.NO_CIRCUIT_SIGNER_KEY;

    const env = await getEnv(store);
    commandRef.current = new Command("start-executor", ["start"], {
      cwd: "../../executor",
      env,
    });
    commandRef.current.on("close", (data) => {
      log("executor", {
        type: "info",
        message: `Executor closed: ${JSON.stringify(data)}`,
      });
      setState(RunningState.Idle);
    });
    commandRef.current.on("error", (error) =>
      log("executor", { type: "error", message: error }),
    );
    commandRef.current.stdout.on("data", (line) =>
      log("executor", { type: "info", message: line }),
    );
    commandRef.current.stderr.on("data", (line) =>
      log("executor", { type: "error", message: line }),
    );
    childProcessRef.current = await commandRef.current.spawn();
    console.log("childProcessRef", childProcessRef.current?.pid);
    log("executor", {
      type: "success",
      message: `Executor started with PID: ${childProcessRef.current.pid}`,
    });
    setState(RunningState.Active);
  };
  const stop = async () => {
    if (state === RunningState.Idle) return;
    childProcessRef.current?.kill();
    setState(RunningState.Idle);
  };
  const restart = async () => {
    if (state === RunningState.Idle) return;
    await stop();
    await start();
  };

  return (
    <ExecutorControlContext.Provider
      value={{
        state,
        start,
        stop,
        restart,
      }}
    >
      {children}
    </ExecutorControlContext.Provider>
  );
};

export const useExectorControl = () => useContext(ExecutorControlContext);
