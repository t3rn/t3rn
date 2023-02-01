import { AmountConverter } from '../../src/converters/amounts';
import { describe, it, expect } from '@jest/globals';
import { BN } from 'bn.js';

describe("AmountConverter @ SDK", () => {
    it("Should create a new AmountConverter instance", () => {
        // Arrange + Act
        const be = new AmountConverter({
            value: 0,
            decimals: 12,
            valueTypeSize: 16
        });
        // Assert
        expect(be).toBeInstanceOf(AmountConverter);
    });

    it("Should correctly convert value to float", () => {
        // Arange
        const be = new AmountConverter({ value: 100 });
        const expectedValue = 100 / (10 ** 12)
        // Act
        const beAsFloat = Number(be.toFloat());
        // Assert
        expect(beAsFloat).toBeCloseTo(expectedValue);
    });

    it("Should correctly convert value to BN", () => {
        // Arange
        const be = new AmountConverter({
            value: 100,
        });
        // Act
        const beAsBn = be.toBn();
        // Assert
        expect(beAsBn).toBeInstanceOf(BN);
    });

    it("Should fail when trying to convert a value is larget than type size", () => {
        // Arange
        const be = new AmountConverter({
            value: 100,
            valueTypeSize: 1,
        });
        // Act
        const safeConversion = be.checkSafeConversion(new BN.BN(100 + 1));
        // Assert
        expect(safeConversion).toBeFalsy();
    });

    it("Should conver to little endian array", () => {
        // Arange
        const be = new AmountConverter({
            value: 100,
            valueTypeSize: 1,
        });
        // Act
        const leArray = be.toLeArray();
        // Assert
        expect(leArray).toBeInstanceOf(Array)
        expect(leArray).toEqual([100]);
    });

    it("Should convert to little endian hex array", () => {
        // Arange
        const be = new AmountConverter({
            value: 100,
            valueTypeSize: 1,
        });
        // Act
        const leHex = be.toLeHex();
        // Assert
        expect(leHex).toEqual("0x64");
    })

    it("Should throw an error when trying to convert a float value", () => {
        expect(() => {
            // Arange
            const be = new AmountConverter({
                value: 100.1,
            });
            // Act
            be.toFloat()
        }).toThrow();  // Assert
    })
})