import { ExecutionLayerType } from "./gateways/types";
import { SfxType, SfxStatus, XtxStatus } from "./side-effects/types";

export const Gateway = {
	ExecutionLayerType
}

/**
 * SideEffect types namespace
 */
export const SideEffect = {
	SfxType,
	SfxStatus,
	XtxStatus
}

export{ SfxType, SfxStatus, XtxStatus, ExecutionLayerType }