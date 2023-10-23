import classNames from "classnames";
import { overrideTailwindClasses } from "tailwind-override";
import { ButtonProps } from "./Button";

export const SecondaryButton = (props: ButtonProps) => {
  return (
    <button
      {...props}
      className={overrideTailwindClasses(
        classNames(
          "border transition border-gray-400 hover:border-gray-600 hover:text-gray-400 p-2 rounded-lg text-gray-300 w-full focus:ring focus:ring-blue-300  focus:outline-none whitespace-nowrap flex items-center justify-center hover:shadow-t3rn disabled:opacity-50 disabled:cursor-not-allowed",
          props.className,
        ),
      )}
    >
      {props.children}
    </button>
  );
};
