import { ReactNode, createContext, useContext, useState } from "react";

export enum View {
  Content,
  Configure,
}

interface ViewContextValue {
  view: View;
  setView: (view: View) => void;
}

export const ViewContext = createContext({} as ViewContextValue);

interface ViewProviderProps {
  children: ReactNode;
}

export const ViewProvider = ({ children }: ViewProviderProps) => {
  const [view, setView] = useState(View.Content);
  return (
    <ViewContext.Provider
      value={{
        view,
        setView,
      }}
    >
      {children}
    </ViewContext.Provider>
  );
};

export const useView = () => useContext(ViewContext);
