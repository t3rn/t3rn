//
//
// pub trait Portal<T: frame_system::Config> {
//     fn register_gateway(
//             origin: OriginFor<T>,
//             url: Vec<u8>,
//             gateway_id: ChainId,
//             gateway_abi: GatewayABIConfig,
//             gateway_vendor: GatewayVendor, // Maps to FV
//             gateway_type: GatewayType,
//             gateway_genesis: GatewayGenesisConfig,
//             gateway_sys_props: GatewaySysProps,
//             allowed_side_effects: Vec<AllowedSideEffect>,
//             registration_data: Vec<u8>
//          ) -> DispatchResultWithPostInfo;
// }