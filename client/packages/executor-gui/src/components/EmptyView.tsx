import { motion } from "framer-motion";
import { overrideTailwindClasses } from "tailwind-override";
import { SecondaryButton } from "./SecondaryButton";

const fadeInPreset = {
  initial: {
    opacity: 0,
  },
  animate: {
    opacity: 1,
  },
};

export interface EmptyViewProps {
  className?: string;
  description: string;
  title: string;
  actionText: string;
  action?(): void;
}

const EmptyView = ({
  title,
  description,
  action,
  actionText,
  className,
}: EmptyViewProps) => {
  return (
    <motion.div
      variants={fadeInPreset}
      initial="initial"
      animate="animate"
      exit="initial"
      className={overrideTailwindClasses(`w-full ${className}`)}
    >
      <h1 className="text-white text-center text-xl">{title}</h1>
      <p className="text-gray-400 text-center max-w-sm table m-auto mt-3">
        {description}
      </p>
      {action && (
        <SecondaryButton onClick={action} className="table w-auto m-auto mt-8">
          {actionText}
        </SecondaryButton>
      )}
    </motion.div>
  );
};

export default EmptyView;

EmptyView.defaultProps = {
  title: "",
  description: "There is nothing to show here...",
  actionText: "Refresh",
};
