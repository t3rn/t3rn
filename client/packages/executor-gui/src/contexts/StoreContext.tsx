import { ReactNode, createContext, useContext } from "react";
import { Store } from "tauri-plugin-store-api";

const store = new Store(".settings.dat");

export const StoreContext = createContext(store);

interface StoreProviderProps {
  children: ReactNode;
}

export const StoreProvider = ({ children }: StoreProviderProps) => {
  return (
    <StoreContext.Provider value={store}>{children}</StoreContext.Provider>
  );
};

export const useStore = () => useContext(StoreContext);
