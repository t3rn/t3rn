import { FormikErrors } from "formik";
import toast from "react-hot-toast";

const useToastFormikErrors = <T,>(
  errors: FormikErrors<T>,
  hintText?: string,
): { toastErrors: () => void } => {
  const getAllErrorMessages = (
    errorNode: string | string[] | Record<string, string>,
    errorMessages: string[] = [],
  ) => {
    if (typeof errorNode === "string") {
      errorMessages.push(errorNode);

      return;
    }

    if (Array.isArray(errorNode)) {
      errorNode.forEach((error) => getAllErrorMessages(error, errorMessages));

      return;
    }

    if (typeof errorNode === "object") {
      Object.values(errorNode).forEach((error) =>
        getAllErrorMessages(error, errorMessages),
      );
    }
  };

  return {
    toastErrors() {
      let messages: string[] = [];

      getAllErrorMessages(Object.values(errors), messages);

      // remove duplicate error messages...
      messages = [...new Set(messages)];

      const prepareHintText = `Please attend to ${messages.length > 1 ? "these errors" : "this error"
        }:`;

      toast.error(
        <div>
          {hintText || prepareHintText}
          <ul>
            {messages.map((error, index) => (
              <li key={index}>&bull; {error}</li>
            ))}
          </ul>
        </div>,
      );
    },
  };
};

export default useToastFormikErrors;
