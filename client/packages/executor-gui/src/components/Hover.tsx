import { useState } from "react";

interface HoverProps {
  children: (isHovered: boolean) => JSX.Element;
  className?: string;
}

export const Hover = ({ children, className }: HoverProps) => {
  const [isHover, setIsHover] = useState(false);

  return (
    <div
      onMouseEnter={() => setIsHover(true)}
      onMouseLeave={() => setIsHover(false)}
      className={className}
    >
      {children(isHover)}
    </div>
  );
};
