#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

use sp_std::collections::btree_map::BTreeMap;

use sp_std::vec::*;

type StateKey = [u8; 32];
type StateVal = Vec<u8>;
// Although check if no longer than 64 bytes
pub type State = BTreeMap<StateKey, StateVal>;

use sp_io::hashing::twox_256;

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode, RuntimeDebug)]
pub struct LocalState {
    pub state: State,
}

impl LocalState {
    pub fn new() -> Self {
        LocalState {
            state: BTreeMap::new(),
        }
    }
}

impl Volatile for LocalState {
    fn get_state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    fn get_state(&self) -> &State {
        &self.state
    }
}

pub trait Volatile {
    fn get_state_mut(&mut self) -> &mut State;

    fn get_state(&self) -> &State;

    fn hash(x: &[u8]) -> [u8; 32] {
        twox_256(x)
    }

    fn key_2_state_key<K: Encode>(key: K) -> [u8; 32] {
        let key_as_array = &key.encode()[..];
        Self::hash(key_as_array)
    }

    fn get<K: Encode>(&self, key: K) -> Option<&StateVal> {
        self.get_state().get(&Self::key_2_state_key(key))
    }

    fn cmp<K: Encode>(&self, key: K, cmp_value: Vec<u8>) -> bool {
        self.get(key) == Some(cmp_value).as_ref()
    }

    // fn value_2_state_value(value: Vec<u8>) -> Result<[u8; 64], &'static str>  {
    fn value_2_state_value(value: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        // let value_as_array = &value[..];
        return if value.len() > 64 {
            Err("Value is larger than max. 64 bytes allowed in the Volatile State")
        } else {
            Ok(value)
        };
    }

    fn insert<K: Encode>(
        &mut self,
        key: K,
        val: Vec<u8>,
    ) -> Result<(StateKey, StateVal), &'static str> {
        let key_candidate = Self::key_2_state_key(key);
        if self.get_state().contains_key(&key_candidate) {
            return Err("Key already exists in the Volatile State");
        }
        let value_candidate = Self::value_2_state_value(val)?;

        match self
            .get_state_mut()
            .insert(key_candidate.clone(), value_candidate.clone())
        {
            Some(_) => Err("Critical ERR - key override in Volatile storage, despite check!"),
            None => Ok((key_candidate, value_candidate)),
        }
    }
}

#[cfg(test)]
pub mod tests {
    pub const FROM_2XX_32B_HASH: [u8; 32] = [
        47u8, 140u8, 44u8, 35u8, 27u8, 124u8, 17u8, 66u8, 71u8, 139u8, 84u8, 182u8, 189u8, 44u8,
        255u8, 9u8, 216u8, 225u8, 72u8, 92u8, 140u8, 153u8, 36u8, 176u8, 243u8, 84u8, 204u8, 37u8,
        235u8, 116u8, 243u8, 46u8,
    ];
    pub const TO_2XX_32B_HASH: [u8; 32] = [
        158u8, 28u8, 58u8, 252u8, 48u8, 192u8, 42u8, 109u8, 5u8, 152u8, 177u8, 124u8, 120u8, 75u8,
        35u8, 127u8, 172u8, 179u8, 9u8, 243u8, 249u8, 54u8, 121u8, 35u8, 254u8, 120u8, 104u8, 24u8,
        148u8, 166u8, 97u8, 122u8,
    ];
    pub const VALUE_2XX_32B_HASH: [u8; 32] = [
        92u8, 190u8, 233u8, 194u8, 88u8, 78u8, 131u8, 17u8, 72u8, 80u8, 231u8, 10u8, 137u8, 245u8,
        128u8, 209u8, 66u8, 12u8, 149u8, 254u8, 145u8, 206u8, 210u8, 112u8, 210u8, 191u8, 191u8,
        97u8, 176u8, 219u8, 170u8, 83u8,
    ];

    use super::*;
    use hex_literal::hex;

    #[test]
    fn successfully_generates_correct_keys_for_inserted_values() {
        let _encoded_transfer_args_input = vec![
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
            1u64.encode(),
        ];
        let mut local_state = LocalState::new();

        let mut res = local_state.insert(
            "from",
            hex!("0909090909090909090909090909090909090909090909090909090909090909").into(),
        );
        assert_eq!(
            res,
            Ok((
                FROM_2XX_32B_HASH,
                hex!("0909090909090909090909090909090909090909090909090909090909090909").into()
            ))
        );

        res = local_state.insert(
            "to",
            hex!("0606060606060606060606060606060606060606060606060606060606060606").into(),
        );
        assert_eq!(
            res,
            Ok((
                TO_2XX_32B_HASH,
                hex!("0606060606060606060606060606060606060606060606060606060606060606").into()
            ))
        );

        res = local_state.insert("value", 1u64.encode());
        assert_eq!(
            res,
            Ok((VALUE_2XX_32B_HASH, hex!("0100000000000000").into()))
        );
    }
}
