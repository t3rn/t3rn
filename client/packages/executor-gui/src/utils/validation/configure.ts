import * as yup from "yup";

export const configureSchema = () =>
  yup.object({
    circuitSignerKey: yup.string().required("Circuit signer key is required"),
    executor: yup.string().required("Executor name is required"),
    logLevel: yup.string().required("Log level is required"),
    logPretty: yup.boolean().required("Log pretty is required"),
    circuitWsEndpoint: yup.string().required("Circuit endpoint is required"),
    ethereumPrivateKey: yup
      .string()
      .required("Ethereum private key is required"),
    relayChainSignerKey: yup
      .string()
      .required("Relay chain signer key is required"),
    processBatches: yup.boolean().required("Process batches is required"),
  });
