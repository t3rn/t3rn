import { Toaster } from "react-hot-toast";
import { Content } from "./Content";
import { Footer } from "./Footer";
import { View, useView } from "@/contexts/ViewContext";
import { Configure } from "./Configure";
import { AnimatePresence } from "framer-motion";

export const AppRoot = () => {
  const { view } = useView();
  return (
    <div className="h-screen flex flex-col">
      <div className="p-3 pb-0">
        <h1 className="font-bold">Executor</h1>
      </div>
      <AnimatePresence>
        {view === View.Content && <Content />}
        {view === View.Configure && <Configure />}
      </AnimatePresence>
      <Footer />
      <Toaster toastOptions={{ className: "toast" }} />
    </div>
  );
};
