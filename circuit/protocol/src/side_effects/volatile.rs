#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;

use sp_std::collections::btree_map::BTreeMap;

use sp_std::vec::*;

type StateKey = [u8; 32];
type StateVal = Vec<u8>; // Although check if no longer than 64 bytes
pub type State = BTreeMap<StateKey, StateVal>;

use sp_io::hashing::twox_256;

pub struct LocalState {
    state: State,
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

    // fn value_2_state_value(value: Vec<u8>) -> Result<[u8; 64], &'static str>  {
    fn value_2_state_value(value: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        // let value_as_array = &value[..];
        return if value.len() > 64 {
            Err("Value is larger than max. 64 bytes allowed in the Volatile State")
        } else {
            Ok(value)
        };
    }

    fn insert<K: Encode>(&mut self, key: K, val: Vec<u8>) -> Result<(), &'static str> {
        let key_candidate = Self::key_2_state_key(key);
        if self.get_state().contains_key(&key_candidate) {
            return Err("Key already exists in the Volatile State");
        }
        let value_candidate = Self::value_2_state_value(val)?;

        match self.get_state_mut().insert(key_candidate, value_candidate) {
            Some(_) => Ok(()),
            None => Err("Can't inser the key to volatile store"),
        }
    }
}
