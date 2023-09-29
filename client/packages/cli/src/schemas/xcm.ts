import { z } from "zod"

export const XcmTransferSchema = z.object({
    dest: z.string({
        invalid_type_error: "Destination chain ID be a string",
        required_error: "Destination chain ID is required",
    })
    .max(4),
    recipient: z.string({
            required_error: "Recipient is required",
            invalid_type_error: "Recipient must be a string",
        }),
    targetAmount: z.number({
        invalid_type_error: "Target amount must be a number",
        required_error: "Target amount is required",
    }),
    targetAsset: z.string({
        invalid_type_error: "Target asset amount must be a string",
        required_error: "Target asset is required",
    }),
})
