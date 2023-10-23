import { ReactNode, createContext, useContext, useState } from "react";

export type LogType = "info" | "error" | "warn" | "debug" | "success";
export const Logs = {
  Info: "info",
  Error: "error",
  Warn: "warn",
  Debug: "debug",
  Success: "success",
} as const;

export interface Log {
  type: LogType;
  message: string;
  timestamp: number;
}

interface Context {
  logs: Map<string, Log[]>;
  log: (label: string, payload: Omit<Log, "timestamp">) => void;
  clear: (label?: string) => void;
}

const LoggerContext = createContext({} as Context);

interface Props {
  children: ReactNode;
}

export const LoggerProvider = ({ children }: Props) => {
  const [logs, setLog] = useState(new Map<string, Log[]>());
  const log = (label: string, payload: Omit<Log, "timestamp">) => {
    setLog((prevLogsTable) => {
      const logsTable = new Map(prevLogsTable);
      const logs = logsTable.has(label) ? (logsTable.get(label) as Log[]) : [];
      logs.push({ ...payload, timestamp: Date.now() });
      logsTable.set(label, logs);
      return logsTable;
    });
  };
  const clear = (label?: string) => {
    if (label) {
      setLog((prevLogsTable) => {
        const clone = new Map(prevLogsTable);
        clone.delete(label);
        return clone;
      });
      return;
    }
    setLog(new Map<string, Log[]>());
  };

  return (
    <LoggerContext.Provider
      value={{
        logs,
        log,
        clear,
      }}
    >
      {children}
    </LoggerContext.Provider>
  );
};

export const useLogger = () => useContext(LoggerContext);
