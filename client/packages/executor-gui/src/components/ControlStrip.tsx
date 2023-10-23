import {
  RunningState,
  useExectorControl,
} from "../contexts/ExecutorControlContext";
import { withResultToast } from "../utils/fns";
import { SecondaryButton } from "./SecondaryButton";
import SvgPlay from "./icons/Play";
import SvgRotateCcw from "./icons/RotateCcw";
import SvgSquare from "./icons/Square";

export const ControlStrip = () => {
  return (
    <div>
      <ControlExecutor />
    </div>
  );
};

const ControlExecutor = () => {
  const { state, start, stop, restart } = useExectorControl();
  const startExecutor = () => withResultToast(start, []);
  const stopExecutor = () =>
    withResultToast(stop, [], "Executor was successfully stopped!");
  const restartExecutor = () => withResultToast(restart);

  return (
    <div className="flex-1 flex gap-3">
      <SecondaryButton
        disabled={state === RunningState.Active}
        onClick={startExecutor}
      >
        <SvgPlay width={16} className="text-green-300" />
      </SecondaryButton>
      <SecondaryButton
        disabled={state === RunningState.Idle}
        onClick={stopExecutor}
      >
        <SvgSquare className="text-red-300" width={16} />
      </SecondaryButton>
      <SecondaryButton
        disabled={state === RunningState.Idle}
        onClick={restartExecutor}
      >
        <SvgRotateCcw width={16} />
      </SecondaryButton>
    </div>
  );
};
