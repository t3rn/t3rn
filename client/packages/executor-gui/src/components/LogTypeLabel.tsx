import { LogType, Logs } from "@/contexts/LoggerContext";
import className from "classnames";

interface LogTypeLabelProps {
  type: LogType;
}

export const LogTypeLabel = ({ type }: LogTypeLabelProps) => {
  return (
    <span
      className={className(
        "text-sm px-1 py-[3px] rounded-lg opacity-70 border",
        {
          "bg-red-200/30 text-red-50 border-red-200": type === Logs.Error,
          "bg-blue-200/30 text-blue-50 border-blue-200": type === Logs.Info,
          "bg-green-200/30 text-green-50 border-green-200":
            type === Logs.Success,
          "bg-yellow-200/30 text-yellow-50 border-yellow-200":
            type === Logs.Warn,
        },
      )}
    >
      {type}
    </span>
  );
};
