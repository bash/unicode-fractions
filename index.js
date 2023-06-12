document.querySelector('#fraction-input').addEventListener('input', event => {
    const input = event.target;
    setValueAndPreserveSelection(input, toUnicodeFraction(splitInput(input.value)));
});

function toUnicodeFraction({ whole, numerator, denominator }) {
    const wholeFormatted = whole == null ? '' : toRegularScript(whole.trimEnd(' ')) + THIN_SPACE;
    return denominator == null
        ? wholeFormatted + toSuperscript(numerator)
        : wholeFormatted + toSuperscript(numerator) + FRACTION_SEPARATOR + toSubscript(denominator);
}

function setValueAndPreserveSelection(input, value) {
    const { selectionStart, selectionEnd } = input;
    Object.assign(input, { value, selectionStart, selectionEnd });
}

function toSuperscript(input) {
    return [...input].map(char => SUPERSCRIPT_MAPPING.get(char) || char).join('');
}

function toSubscript(input) {
    return [...input].map(char => SUBSCRIPT_MAPPING[char] || char).join('');
}

function toRegularScript(input) {
    return [...input].map(char => SUPERSCRIPT_REVERSE_MAPPING.get(char) || SUBSCRIPT_REVERSE_MAPPING.get(char) || char).join('');
}

function splitInput(input) {
    const [whole, fraction] = splitAtLastSpace(input);
    const [numerator, denominator] = splitAtSlash(fraction);
    return { whole, numerator, denominator };
}

function splitAtLastSpace(input) {
    const index = lastIndexOfAny(input, ' ', THIN_SPACE);
    return index != -1
        ? [input.substring(0, index + 1), input.substring(index + 1, input.length)]
        : [null, input];
}

function splitAtSlash(input) {
    const index = indexOfAny(input, '/', FRACTION_SEPARATOR);
    return index != -1
        ? [input.substring(0, index), input.substring(index + 1, input.length)]
        : [input];
}

function indexOfAny(haystack, ...needles) {
    const indexes = needles
        .map(needle => haystack.indexOf(needle))
        .filter(index => index != -1);
    return indexes.length >= 1 ? Math.min(...indexes) : -1;
}

function lastIndexOfAny(haystack, ...needles) {
    const indexes = needles
        .map(needle => haystack.lastIndexOf(needle))
        .filter(index => index != -1);
    return indexes.length >= 1 ? Math.max(...indexes) : -1;
}

const THIN_SPACE = ' ';

const FRACTION_SEPARATOR = '⁄';

const SUPERSCRIPT_MAPPING = new Map([
    ['-', '⁻'],
    ['+', '⁺'],
    ['0', '⁰'],
    ['1', '¹'],
    ['2', '²'],
    ['3', '³'],
    ['4', '⁴'],
    ['5', '⁵'],
    ['6', '⁶'],
    ['7', '⁷'],
    ['8', '⁸'],
    ['9', '⁹'],
]);

const SUPERSCRIPT_REVERSE_MAPPING = flip(SUPERSCRIPT_MAPPING);

const SUBSCRIPT_MAPPING = {
    '-': '₋',
    '+': '₊',
    '0': '₀',
    '1': '₁',
    '2': '₂',
    '3': '₃',
    '4': '₄',
    '5': '₅',
    '6': '₆',
    '7': '₇',
    '8': '₈',
    '9': '₉',
};

const SUBSCRIPT_REVERSE_MAPPING = flip(SUBSCRIPT_MAPPING);

function flip(map) {
    return new Map(Array.from(map).map(([a, b]) => [b, a]));
}
