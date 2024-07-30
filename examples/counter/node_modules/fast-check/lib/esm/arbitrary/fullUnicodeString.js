import { array } from './array.js';
import { fullUnicode } from './fullUnicode.js';
import { codePointsToStringMapper, codePointsToStringUnmapper } from './_internals/mappers/CodePointsToString.js';
import { createSlicesForString } from './_internals/helpers/SlicesForStringBuilder.js';
const safeObjectAssign = Object.assign;
export function fullUnicodeString(constraints = {}) {
    const charArbitrary = fullUnicode();
    const experimentalCustomSlices = createSlicesForString(charArbitrary, codePointsToStringUnmapper);
    const enrichedConstraints = safeObjectAssign(safeObjectAssign({}, constraints), {
        experimentalCustomSlices,
    });
    return array(charArbitrary, enrichedConstraints).map(codePointsToStringMapper, codePointsToStringUnmapper);
}
