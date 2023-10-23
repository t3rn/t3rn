import { FC, ReactNode, useState } from "react";
import classNames from "classnames";
import { motion } from "framer-motion";
import * as ReactSelect from "react-select";
import {
  components,
  OptionProps,
  Props,
  DropdownIndicatorProps,
  SingleValueProps,
  SingleValue as Single,
} from "react-select";
import ChevronDown from "./icons/ChevronDown";

interface Data {
  icon?: ReactNode;
}

const DropdownIndicator = (props: DropdownIndicatorProps<Data>) => {
  return (
    <components.DropdownIndicator {...props}>
      <ChevronDown width={10} color="currentColor" />
    </components.DropdownIndicator>
  );
};

const SingleValue = ({ children, ...props }: SingleValueProps<Data>) => {
  return (
    <components.SingleValue {...props}>
      <div className="flex items-center gap-3">
        {props.data?.icon && <div className="flex">{props.data.icon}</div>}
        {children}
      </div>
    </components.SingleValue>
  );
};

type ExtendedOption<T> = OptionProps<T> & { selectProps: Props<T> };

const Option = ({ children, ...props }: ExtendedOption<Data>) => {
  return (
    <components.Option {...props}>
      <div className="flex items-center gap-3">
        {props?.data?.icon && (
          <div className="flex mr-[3px]">{props.data.icon}</div>
        )}
        {children}
      </div>
    </components.Option>
  );
};

interface SelectOption {
  label: string;
  value: string;
  icon?: ReactNode;
}

export type SelectProps = Omit<
  Props,
  "onChange" | "defaultValue" | "options"
> & {
  width?: string;
  height?: string;
  label?: string;
  error?: string | ReactNode;
  touched?: boolean;
  focusIndicator?: boolean;
  onChange: (event: React.ChangeEvent) => void;
  options: SelectOption[];
  defaultValue?: string;
};

const Select: FC<SelectProps> = (props: SelectProps) => {
  const { label, touched, error, onChange } = props;
  const [focused, setFocused] = useState(false);
  const defaultValue = props?.defaultValue
    ? props.options.find((option) => option.value === props?.defaultValue)
    : null;
  const handleChange = (single: Single<Data>) => {
    onChange({
      target: { name: props.name, value: (single as { value: string }).value },
    } as never);
  };
  const handleBlur = () => {
    props.onBlur && props.onBlur({ target: { name: props.name } } as never);
  };
  return (
    <div
      style={{ width: props?.width }}
      className={classNames("relative", "select-input", {
        "menu-open": focused,
        "no-focus-indicator": !props.focusIndicator,
      })}
    >
      {label && (
        <label htmlFor={props.id} className="block text-md mb-1 text-gray-500">
          {label}
        </label>
      )}
      <div style={{ marginTop: "7px" }} className="relative">
        <ReactSelect.default<Data>
          classNamePrefix="select"
          // eslint-disable-next-line @typescript-eslint/ban-ts-comment
          //@ts-ignore
          components={{ DropdownIndicator, SingleValue, Option }}
          isSearchable={props?.isSearchable || false}
          {...props}
          onMenuOpen={() => {
            setFocused(true);
          }}
          onMenuClose={() => {
            setFocused(false);
          }}
          onChange={handleChange}
          onBlur={handleBlur}
          defaultValue={defaultValue}
        />

        {props.focusIndicator && (
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
        )}
      </div>
      {touched && error && (
        <div className="text-red-500 text-sm mt-2">{error}</div>
      )}
    </div>
  );
};

export default Select;

Select.defaultProps = {
  focusIndicator: true,
};
