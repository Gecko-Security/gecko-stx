import { Value } from '../../../check/arbitrary/definition/Value.js';
import { cloneMethod } from '../../../check/symbols.js';
import { stringify, toStringMethod } from '../../../utils/stringify.js';
export function buildGeneratorValue(mrng, biasFactor, computePreBuiltValues, arbitraryCache) {
    const preBuiltValues = computePreBuiltValues();
    let localMrng = mrng.clone();
    const context = { mrng: mrng.clone(), biasFactor, history: [] };
    const valueFunction = (arb) => {
        const preBuiltValue = preBuiltValues[context.history.length];
        if (preBuiltValue !== undefined && preBuiltValue.arb === arb) {
            const value = preBuiltValue.value;
            context.history.push({ arb, value, context: preBuiltValue.context, mrng: preBuiltValue.mrng });
            localMrng = preBuiltValue.mrng.clone();
            return value;
        }
        const g = arb.generate(localMrng, biasFactor);
        context.history.push({ arb, value: g.value_, context: g.context, mrng: localMrng.clone() });
        return g.value;
    };
    const memoedValueFunction = (arb, ...args) => {
        return valueFunction(arbitraryCache(arb, args));
    };
    const valueMethods = {
        values() {
            return context.history.map((c) => c.value);
        },
        [cloneMethod]() {
            return buildGeneratorValue(mrng, biasFactor, computePreBuiltValues, arbitraryCache).value;
        },
        [toStringMethod]() {
            return stringify(context.history.map((c) => c.value));
        },
    };
    const value = Object.assign(memoedValueFunction, valueMethods);
    return new Value(value, context);
}
