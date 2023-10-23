import { InputHTMLAttributes, useState, forwardRef, ChangeEvent } from "react";
import classNames from "classnames";
import Cleave from "cleave.js/react";
import { motion } from "framer-motion";
import { CleaveOptions } from "cleave.js/options";

export type InputProps = {
  touched?: boolean;
  error?: string | string[];
  success?: string | boolean;
  label?: string | JSX.Element;
  formGroupClass?: string;
  mask?: boolean;
  maskOptions?: CleaveOptions;
} & InputHTMLAttributes<HTMLInputElement>;

const Input = forwardRef<HTMLInputElement, InputProps>((props, ref) => {
  const [focused, setFocused] = useState(false);

  const { touched, formGroupClass, mask, maskOptions, onChange, ...others } =
    props;

  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    let currentValue = event.target.value;

    if (maskOptions?.prefix) {
      currentValue = currentValue.replace(maskOptions.prefix, "");
    }

    if (maskOptions?.numeral) {
      currentValue = currentValue.replace(/,/g, "");
    }

    const newevent = { ...event };
    newevent.target.value = currentValue;

    if (onChange) onChange(newevent);
  };

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    setFocused(true);
    if (others.onFocus) others.onFocus(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    setFocused(false);
    if (others.onBlur) others.onBlur(e);
  };

  return (
    <div className={classNames("mb-4 w-full relative", formGroupClass)}>
      {props.label && (
        <label htmlFor={props.id} className="block  mb-1 text-gray-500">
          {props.label}
        </label>
      )}
      <div className="relative">
        {mask ? (
          // eslint-disable-next-line @typescript-eslint/ban-ts-comment
          // @ts-ignore
          <Cleave
            {...others}
            htmlRef={ref as never}
            className={classNames(
              "bg-transparent w-full py-2 border-b border-gray-700 text-white outline-none focus:border-none rounded-none",
              props.className,
            )}
            onChange={handleChange}
            onFocus={handleFocus}
            onBlur={handleBlur}
            options={maskOptions ?? {}}
          />
        ) : (
          <div className="relative">
            <input
              {...others}
              className={classNames(
                "bg-transparent w-full py-2 border-b border-gray-700 text-white outline-none focus:border-none rounded-none",
                props.className,
              )}
              onChange={onChange}
              onFocus={(e) => {
                setFocused(true);
                if (others.onFocus) others.onFocus(e);
              }}
              onBlur={(e) => {
                setFocused(false);
                if (others.onBlur) others.onBlur(e);
              }}
              ref={ref}
            />
          </div>
        )}

        <motion.div
          className="bg-indigo-400 absolute z-10"
          initial={{ width: 0, left: "50%", bottom: "1px" }}
          animate={{
            width: focused ? "100%" : 0,
            left: focused ? 0 : "50%",
          }}
          transition={{ type: "keyframes" }}
          style={{ height: "1px" }}
        ></motion.div>
      </div>

      {touched && props.error && (
        <div className="text-red-500 text-sm mt-2">{props.error}</div>
      )}
    </div>
  );
});

Input.displayName = "Input";

export default Input;

export type RangeSliderProps = InputProps &
  InputHTMLAttributes<HTMLInputElement> & {
    steps?: Array<{ name: string; description: string }>;
  };

export const RangeSlider = (props: RangeSliderProps) => {
  const { label, formGroupClass, steps, ...others } = props;
  const midValue = steps?.length ?? 0 / 2;
  const currentStep = parseInt(others.value as string);

  return (
    <div className={formGroupClass}>
      <label className="text-white">{label}</label>
      <div className="mt-1">
        <div className="range">
          <div
            className={classNames("range--state range__low", {
              [`opacity-${midValue > currentStep ? 100 : 0}`]: true,
            })}
          ></div>
          <div
            className={classNames("range--state range__middle", {
              [`opacity-${midValue === currentStep ? 100 : 0}`]: true,
            })}
          ></div>
          <div
            className={classNames("range--state range__high", {
              [`opacity-${midValue < currentStep && currentStep !== steps?.length
                  ? 100
                  : 0
                }`]: true,
            })}
          ></div>
          <div
            className={classNames("range--state range__max", {
              [`opacity-${currentStep === steps?.length ? 100 : 0}`]: true,
            })}
          ></div>
          <input type="range" {...others} />
        </div>
        {steps && (
          <div className="grid grid-cols-4 select-none">
            {steps.map(({ name, description }, index) => (
              <div
                key={index}
                className={classNames(
                  {
                    "text-gray-500": index + 1 !== others.value,
                    "text-white": index + 1 === others.value,
                    "text-left": index === 0,
                    "text-center": index > 0 && index < steps.length - 1,
                    "text-right": index + 1 === steps.length,
                  },
                  "transition",
                  "range-step",
                )}
              >
                <h1 className="text-lg mb-0">{name}</h1>
                <p className=" text-xs">{description}</p>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
