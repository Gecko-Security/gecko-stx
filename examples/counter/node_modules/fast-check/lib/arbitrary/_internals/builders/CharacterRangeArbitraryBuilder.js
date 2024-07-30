"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.buildAlphaNumericPercentArbitrary = exports.buildAlphaNumericArbitrary = exports.buildLowerAlphaNumericArbitrary = exports.buildLowerAlphaArbitrary = void 0;
const fullUnicode_1 = require("../../fullUnicode");
const oneof_1 = require("../../oneof");
const mapToConstant_1 = require("../../mapToConstant");
const globals_1 = require("../../../utils/globals");
const safeStringFromCharCode = String.fromCharCode;
const lowerCaseMapper = { num: 26, build: (v) => safeStringFromCharCode(v + 0x61) };
const upperCaseMapper = { num: 26, build: (v) => safeStringFromCharCode(v + 0x41) };
const numericMapper = { num: 10, build: (v) => safeStringFromCharCode(v + 0x30) };
function percentCharArbMapper(c) {
    const encoded = (0, globals_1.encodeURIComponent)(c);
    return c !== encoded ? encoded : `%${(0, globals_1.safeNumberToString)((0, globals_1.safeCharCodeAt)(c, 0), 16)}`;
}
function percentCharArbUnmapper(value) {
    if (typeof value !== 'string') {
        throw new Error('Unsupported');
    }
    const decoded = decodeURIComponent(value);
    return decoded;
}
const percentCharArb = (0, fullUnicode_1.fullUnicode)().map(percentCharArbMapper, percentCharArbUnmapper);
const buildLowerAlphaArbitrary = (others) => (0, mapToConstant_1.mapToConstant)(lowerCaseMapper, { num: others.length, build: (v) => others[v] });
exports.buildLowerAlphaArbitrary = buildLowerAlphaArbitrary;
const buildLowerAlphaNumericArbitrary = (others) => (0, mapToConstant_1.mapToConstant)(lowerCaseMapper, numericMapper, { num: others.length, build: (v) => others[v] });
exports.buildLowerAlphaNumericArbitrary = buildLowerAlphaNumericArbitrary;
const buildAlphaNumericArbitrary = (others) => (0, mapToConstant_1.mapToConstant)(lowerCaseMapper, upperCaseMapper, numericMapper, { num: others.length, build: (v) => others[v] });
exports.buildAlphaNumericArbitrary = buildAlphaNumericArbitrary;
const buildAlphaNumericPercentArbitrary = (others) => (0, oneof_1.oneof)({ weight: 10, arbitrary: (0, exports.buildAlphaNumericArbitrary)(others) }, { weight: 1, arbitrary: percentCharArb });
exports.buildAlphaNumericPercentArbitrary = buildAlphaNumericPercentArbitrary;
