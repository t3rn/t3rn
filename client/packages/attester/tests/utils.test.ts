import { expect } from 'chai'
import { mock } from 'ts-mockito';
import { checkKeys } from "../src/utils";
import { pino } from 'pino';

jest.mock('pino', () => {
    return jest.fn(() => ({
      info: jest.fn(),
      error: jest.fn(),
    }));
  });

describe('Attester', () => {

    beforeEach(() => {
        jest.spyOn(process, 'exit').mockImplementation((code?: number) => {
          throw new Error(`Process exited with code ${code}`);
        });
      });
    
      afterEach(() => {
        jest.restoreAllMocks();
      });
    
    it('should have valid keys', () => {
        const keys = {
            btc: { publicKey: 'btc_public_key' },
            ethereum: { publicKey: 'ethereum_public_key' },
            substrate: { publicKey: 'substrate_public_key' },
        }

        expect(() => {
            checkKeys(keys);
        }).to.not.throw();
    })

    it('should throw on invalid keys', () => {
        const keys = ""

        expect(() => {
            checkKeys(keys); 
          }).to.throw();
    })
})
