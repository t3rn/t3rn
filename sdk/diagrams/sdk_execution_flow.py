

### SDK current flow, state machine is provided by the library

def get_state(xtx_id):
    return []

def submit(new_side_effects):
    pass

def validate_state(state):
    pass

def signal(param):
    return param

def execute(xtx_id, userf):
    state = get_state(xtx_id)

    # dummy to imply validation on state
    validate_state(state)

    if userf(state):
        side_effects_from_userf = [];

        if side_effects_from_userf != 0:
            submit(side_effects_from_userf)
        else:
            execute(xtx_id, userf)
    else:
        return signal("shouldkill")

def example_userf(state):
    side_effects = []

    if state.index % 2 == 0:
        side_effects.append(["dotswap"])

    if state.index == 0:
        side_effects.append(["dottransfer"])
    elif state.index == 4:
        return "shouldkill"
    return side_effects


# #[message]
def t3rn_flip(xtx_id):
    while execute(example_userf) != "shouldkill":
        print("t3rn_flip")


# What if SDK has a registered hook instead? This then call through to the runtime to do everything.
# The runtime would then call the contracts hook once it gets the state to retrieve the next side effects.

# See https://github.com/Supercolony-net/openbrush-contracts#default-implementation-in-ink-traits
# #[t3rn_hook(ctor = "45390", invoke_now = false]]
# fn user_f(state: &[u8]) -> Vec<Vec<u8>>;
# 3vm::register_hook(contract_address, function_id, invoke_now)
# register_hook then starts the execute flow, if invoke now, otherwise it will be on call to contract      <<< can a contract handle multiple calls at the same time? reentrancy, is it a risk?
# on every get_state it then calls the hook(via bare_call(selector|state)) to get the next side effects.
# If the hook returns no more side effects or an error, the runtime would tell the circuit to stop


#
# Such as a handler which takes a state and returns a set of side effects from provided localctx?
#   handler! {
#       if state.index % 2 == 0:
#         side_effects.append(["dotswap"])
#
#           if state.index == 0:
#               side_effects.append(["dottransfer"])
#           elif state.index == 4:
#               return "shouldkill"
#       return side_effects
#   }
#   Or it can be a proc macro attribute on a function the user sets?
#   Then the macro sets up a chain extension with a function id we can define



# Then we can do async execution via 3vm::bare_call