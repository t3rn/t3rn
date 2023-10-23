import { AnimatedCogIcon } from "./icons/AnimatedCogIcon";
import { ControlStrip } from "./ControlStrip";
import { SecondaryButton } from "./SecondaryButton";
import { Hover } from "./Hover";
import { View, useView } from "@/contexts/ViewContext";

export const Footer = () => {
  const { view, setView } = useView();
  const handleConfigureClick = () => setView(View.Configure);

  return (
    <footer className="flex items-center justify-between border-t border-white/10 px-3 py-2">
      <ControlStrip />
      <Hover>
        {(isHover) => (
          <SecondaryButton
            onClick={handleConfigureClick}
            disabled={view === View.Configure}
          >
            <AnimatedCogIcon animate={isHover} width={16} />
          </SecondaryButton>
        )}
      </Hover>
    </footer>
  );
};
