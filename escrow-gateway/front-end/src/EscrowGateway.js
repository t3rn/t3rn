import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  // The currently stored value
  const [currentValue, setCurrentValue] = useState(0);
  const [formValue, setFormValue] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.escrowGateway.something(newValue => {
      // The storage value is an Option<u32>
      // So we have to check whether it is None first
      // There is also unwrapOr
      if (newValue.isNone) {
        setCurrentValue('<None>');
      } else {
        setCurrentValue(newValue.unwrap().toNumber());
      }
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [api.query.escrowGateway]);

  return (
    <Grid.Column width={8}>
      <h1>Escrow Gateway Module</h1>
      <Card centered>
        <Card.Content textAlign='center'>
          <Statistic
            label='Current Value'
            value={currentValue}
          />
        </Card.Content>
      </Card>
      <Form>
        <Form.Field>
          <Input
            label='New Value'
            state='newValue'
            type='number'
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Store Something'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'escrowGateway',
              callable: 'doSomething',
              inputParams: [formValue],
              paramFields: [true]
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  );
}

export default function EscrowGateway (props) {
  const { api } = useSubstrate();
  return (api.query.escrowGateway && api.query.escrowGateway.something
    ? <Main {...props} /> : null);
}
