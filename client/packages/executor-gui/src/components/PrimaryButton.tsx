import classNames from "classnames";
import { overrideTailwindClasses } from "tailwind-override";
import PrimaryGradient from "../gradients/PrimaryGraident";
import { ButtonProps } from "./Button";

export const PrimaryButton = (props: ButtonProps) => {
  return (
    <button
      {...props}
      className={overrideTailwindClasses(
        classNames(
          "relative z-0 bg-transparent px-5 py-2 rounded-lg text-gray-800 w-full focus:ring focus:ring-blue-300 focus:outline-none whitespace-nowrap flex items-center justify-center hover:text-gray-900 disabled:opacity-50 disabled:cursor-not-allowed overflow-hidden group hover:shadow-t3rn-electric transition-all",
          props.className,
        ),
      )}
    >
      {props.children}
    </button>
  );
};
