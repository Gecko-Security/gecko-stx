import { array } from './array.js';
import { hexa } from './hexa.js';
import { codePointsToStringMapper, codePointsToStringUnmapper } from './_internals/mappers/CodePointsToString.js';
import { createSlicesForString } from './_internals/helpers/SlicesForStringBuilder.js';
const safeObjectAssign = Object.assign;
function hexaString(constraints = {}) {
    const charArbitrary = hexa();
    const experimentalCustomSlices = createSlicesForString(charArbitrary, codePointsToStringUnmapper);
    const enrichedConstraints = safeObjectAssign(safeObjectAssign({}, constraints), {
        experimentalCustomSlices,
    });
    return array(charArbitrary, enrichedConstraints).map(codePointsToStringMapper, codePointsToStringUnmapper);
}
export { hexaString };
