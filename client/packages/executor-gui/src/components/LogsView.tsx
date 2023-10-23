import { Log, useLogger } from "@/contexts/LoggerContext";
import { useEffect, useMemo, useRef } from "react";
import SvgTrash from "./icons/Trash";
import { LogTypeLabel } from "./LogTypeLabel";
import EmptyView from "./EmptyView";
import { useVirtualizer } from "@tanstack/react-virtual";

const Header = () => {
  const { clear } = useLogger();
  return (
    <div className="flex items-center justify-between">
      <span>Logs</span>
      <button
        onClick={() => clear()}
        className="flex gap-1 text-gray-400 hover:text-gray-200"
      >
        <SvgTrash width={16} />
        <span className="text-sm">Clear</span>
      </button>
    </div>
  );
};

export const LogsView = () => {
  return (
    <div className="flex-1 flex flex-col gap-2">
      <Header />
      <List label="executor" autoScroll />
    </div>
  );
};

interface ListProps {
  label: string;
  autoScroll?: boolean;
}

const List = ({ label, autoScroll = true }: ListProps) => {
  const { logs } = useLogger();
  const list = useMemo(
    () => (logs.has(label) ? (logs.get(label) as Log[]) : []),
    [logs],
  );
  const count = list.length;
  const ref = useRef<HTMLDivElement>(null);
  const { getTotalSize, getVirtualItems, scrollToIndex, measureElement } =
    useVirtualizer({
      count,
      getScrollElement: () => ref.current,
      estimateSize: () => 35,
    });
  const totalSize = getTotalSize();
  const items = getVirtualItems();

  useEffect(() => {
    if (!autoScroll || count === 0) return;
    scrollToIndex(count - 1, {
      behavior: "auto",
    });
  }, [logs]);

  return (
    <div className="flex-1 relative bg-white/5 rounded-lg">
      {list.length === 0 && (
        <div className="w-full h-full flex items-center">
          <EmptyView title="Oh no logs yet" />
        </div>
      )}

      <div ref={ref} className="absolute inset-0 overflow-y-auto p-2">
        <div
          style={{
            height: `${totalSize}px`,
            width: "100%",
            position: "relative",
          }}
        >
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              width: "100%",
              transform: `translateY(${items[0]?.start ?? 0}px)`,
            }}
          >
            {items.map((virtualRow) => {
              const log = list[virtualRow.index];
              return (
                <div
                  key={virtualRow.key}
                  data-index={virtualRow.index}
                  ref={measureElement}
                  className="pb-2"
                >
                  <LogTypeLabel type={log.type} /> {log.message} {" at "}
                  {new Date(log.timestamp).toLocaleTimeString()}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
};
