#[cfg(test)]
mod tests {
    use crate::*;
    use core::primitive::str;
    use sp_core::U256;

    #[test]
    fn composable_execution_request_should_serialize_deserialize_properly() {
        type Req = InterExecRequest<String, u128>;
        let req: Req = serde_json::from_str(
            r#"
		{
			"origin": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
			"components": [],
			"io": "component1, component2 | component3;",
			"gasLimit": 1000000000000,
			"inputData": "0x8c97db39"
		}
		"#,
        )
        .unwrap();
        assert_eq!(req.gas_limit.into_u256(), U256::from(0xe8d4a51000u64));
        // Deserialize io schedule from string to vec<u8>
        let io_vec: Vec<u8> = req.io.into_boxed_bytes().to_vec();
        assert_eq!(
            io_vec,
            vec![
                99, 111, 109, 112, 111, 110, 101, 110, 116, 49, 44, 32, 99, 111, 109, 112, 111,
                110, 101, 110, 116, 50, 32, 124, 32, 99, 111, 109, 112, 111, 110, 101, 110, 116,
                51, 59
            ]
        );
        // Serialize io schedule from Vec<u8> to string again with core::str
        let io_vec_back_to_str: &str = core::str::from_utf8(io_vec.as_slice()).unwrap();
        assert_eq!(io_vec_back_to_str, "component1, component2 | component3;");
    }

    #[test]
    fn compose_of_request_should_serialize_deserialize_properly() {
        type Req = InterExecRequest<String, u128>;
        let req: Req = serde_json::from_str(
            r#"
		{
			"origin": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
			"components": [{
                "name": "component1",
                "codeTxt": "let a = \"hello\"",
                "gatewayId": [99, 105, 114, 99],
                "execType": "exec-volatile",
                "dest": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
                "value": 0,
                "bytes": "0x8c97db398c97db398c97db398c97db39",
                "inputData": "0x00"
			}],
			"io": "component1, component2 | component3;",
			"gasLimit": 1000000000000,
			"inputData": "0x8c97db39"
		}
		"#,
        )
        .unwrap();
        // Deserializes string fields correctly
        let name_str: &str = &req.components[0].name;
        assert_eq!(name_str, "component1");
        let code_str: &str = &req.components[0].code_txt;
        assert_eq!(code_str, "let a = \"hello\"");
        let exec_type_str: &str = &req.components[0].exec_type;
        assert_eq!(exec_type_str, "exec-volatile");
    }
}
