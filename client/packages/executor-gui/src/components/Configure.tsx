import { useEffect, useState } from "react";
import { useFormik } from "formik";
import toast from "react-hot-toast";
import { SecondaryButton } from "./SecondaryButton";
import SvgSave from "./icons/Save";
import { configureSchema } from "@/utils/validation/configure";
import useToastFormikErrors from "@/hooks/useToastFormikErrors";
import Input from "./Input";
import Select from "./Select";
import { View, useView } from "@/contexts/ViewContext";
import SvgChevronDown from "./icons/ChevronDown";
import { motion } from "framer-motion";
import { useStore } from "@/contexts/StoreContext";
import {
  CIRCUIT_SIGNER_KEY,
  CIRCUIT_WS_ENDPOINT,
  ETHEREUM_PRIVATE_KEY,
  EXECUTOR,
  LOG_LEVEL,
  LOG_PRETTY,
  PROCESS_BATCHES,
  RELAYCHAIN_SIGNER_KEY,
} from "@/consts";
import { Config, getConfig } from "@/utils/config";

const fadeInPreset = {
  initial: {
    opacity: 0,
    x: 5,
  },
  animate: {
    opacity: 1,
    x: 0,
  },
};

export const Configure = () => {
  const store = useStore();
  const [config, setConfig] = useState<Config>();
  useEffect(() => {
    getConfig(store).then(setConfig);
  }, []);

  return (
    <motion.div
      variants={fadeInPreset}
      initial="initial"
      animate="animate"
      exit="initial"
      className="p-3 flex-1 flex flex-col"
    >
      {config && <ConfigureView config={config} />}
    </motion.div>
  );
};

interface ConfigureViewProps {
  config: Config;
}

const ConfigureView = ({ config }: ConfigureViewProps) => {
  const store = useStore();
  const { setView } = useView();
  const circuitSignerKey = config.circuitSignerKey ?? "";
  const executor = config?.executor ?? "My Executor";
  const logLevel = config?.logLevel ?? "DEBUG";
  const logPretty = config?.logPretty ?? "true";
  const circuitWsEndpoint = config?.circuitWsEndpoint ?? "wss://t0rn.io";
  const ethereumPrivateKey = config?.ethereumPrivateKey ?? "";
  const relayChainSignerKey = config?.relayChainSignerKey ?? "";
  const processBatches = config?.processBatches ?? "";
  const {
    values,
    errors,
    touched,
    isValid,
    handleSubmit,
    handleChange,
    handleBlur,
  } = useFormik({
    initialValues: {
      circuitSignerKey,
      executor,
      logLevel,
      logPretty,
      circuitWsEndpoint,
      ethereumPrivateKey,
      relayChainSignerKey,
      processBatches,
    },
    validationSchema: configureSchema,
    onSubmit(formData) {
      (async () => {
        const {
          executor,
          logLevel,
          logPretty,
          circuitSignerKey,
          circuitWsEndpoint,
          ethereumPrivateKey,
          relayChainSignerKey,
          processBatches,
        } = formData;
        await store.set(EXECUTOR, executor);
        await store.set(LOG_LEVEL, logLevel);
        await store.set(LOG_PRETTY, logPretty);
        await store.set(CIRCUIT_SIGNER_KEY, circuitSignerKey);
        await store.set(CIRCUIT_WS_ENDPOINT, circuitWsEndpoint);
        await store.set(ETHEREUM_PRIVATE_KEY, ethereumPrivateKey);
        await store.set(RELAYCHAIN_SIGNER_KEY, relayChainSignerKey);
        await store.set(PROCESS_BATCHES, processBatches);
        await store.save();
        setView(View.Content);
        toast.success("Configuration updated!");
      })();
    },
  });

  const { toastErrors } = useToastFormikErrors(errors);

  const boolOpts = [
    {
      value: "true",
      icon: null,
      label: "Yes",
    },
    {
      value: "false",
      icon: null,
      label: "No",
    },
  ];
  const logLevelsOpts = [
    {
      value: "debug",
      icon: null,
      label: "Debug",
    },
    {
      value: "warn",
      icon: null,
      label: "Warn",
    },
    {
      value: "info",
      icon: null,
      label: "Info",
    },
    {
      value: "trace",
      icon: null,
      label: "Trace",
    },
  ];

  return (
    <form onSubmit={handleSubmit} className="flex-1 flex flex-col gap-2">
      <div className="flex items-center justify-between border-b border-white/10 pb-3">
        <span>Configure</span>
        <div className="flex items-center gap-3">
          <SecondaryButton
            className="flex items-center gap-2 px-3"
            onClick={() => {
              if (!isValid) {
                toastErrors();
              }
            }}
          >
            <SvgSave width={18} className="translate-y-[-1px]" />
            <span className="text-sm">Save</span>
          </SecondaryButton>
          <SecondaryButton
            className="flex items-center gap-1 px-2 py-[9px]"
            onClick={() => setView(View.Content)}
          >
            <SvgChevronDown
              width={18}
              className="-rotate-90 translate-y-[-1px]"
            />
          </SecondaryButton>
        </div>
      </div>

      <div className="flex-1 relative">
        <div className="absolute inset-0 overflow-y-auto px-5 pb-40">
          <Input
            type="text"
            name="executor"
            label="Executor name"
            placeholder="Name your executor"
            value={values.executor}
            error={errors.executor}
            touched={touched.executor}
            onBlur={handleBlur}
            onChange={handleChange}
            autoComplete="off"
          />
          <Input
            type="text"
            name="circuitSignerKey"
            label="Circuit signer key"
            placeholder="Enter key"
            value={values.circuitSignerKey}
            error={errors.circuitSignerKey}
            touched={touched.circuitSignerKey}
            onBlur={handleBlur}
            onChange={handleChange}
            autoComplete="off"
          />
          <Select
            placeholder="Set log level"
            name="logLevel"
            label="Log level"
            width="100%"
            options={logLevelsOpts}
            touched={touched.logLevel}
            error={errors.logLevel}
            onBlur={handleBlur}
            onChange={handleChange}
            defaultValue={logLevel}
          />
          <div className="my-3">
            <Select
              placeholder="Pretty log"
              name="logPretty"
              label="Pretty logs"
              width="100%"
              options={boolOpts}
              touched={touched.logPretty}
              error={errors.logPretty}
              onBlur={handleBlur}
              onChange={handleChange}
              defaultValue={logPretty}
            />
          </div>
          <Input
            type="url"
            name="circuitWsEndpoint"
            label="Circuit RPC endpoint"
            placeholder="Enter RPC endpoint"
            value={values.circuitWsEndpoint}
            error={errors.circuitWsEndpoint}
            touched={touched.circuitWsEndpoint}
            onBlur={handleBlur}
            onChange={handleChange}
            autoComplete="off"
          />
          <Input
            type="text"
            name="ethereumPrivateKey"
            label="Ethereum private key"
            placeholder="Enter ethereum private key"
            value={values.ethereumPrivateKey}
            error={errors.ethereumPrivateKey}
            touched={touched.ethereumPrivateKey}
            onBlur={handleBlur}
            onChange={handleChange}
            autoComplete="off"
          />
          <Input
            type="text"
            name="relayChainSignerKey"
            label="Relay chain signer key"
            placeholder="Enter replay chain signer key"
            value={values.relayChainSignerKey}
            error={errors.relayChainSignerKey}
            touched={touched.relayChainSignerKey}
            onBlur={handleBlur}
            onChange={handleChange}
            autoComplete="off"
          />
          <div className="my-3">
            <Select
              placeholder="Process batches"
              name="processBatches"
              label="Process batches"
              width="100%"
              options={boolOpts}
              touched={touched.processBatches}
              error={errors.processBatches}
              onBlur={handleBlur}
              onChange={handleChange}
              defaultValue={processBatches}
            />
          </div>
        </div>
      </div>
    </form>
  );
};
