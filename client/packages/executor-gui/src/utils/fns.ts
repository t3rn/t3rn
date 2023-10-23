import { toast } from "react-hot-toast";

export const withResultToast = async (
  call: (...args: Array<unknown>) => Promise<unknown>,
  args: Array<unknown> = [],
  successMsg?: string,
) => {
  try {
    await call(...args);
    if (successMsg) toast.success(successMsg);
  } catch (e) {
    toast.error(e as string);
  }
};
