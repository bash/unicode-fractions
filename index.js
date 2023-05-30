document.querySelector('#fraction-input').addEventListener('input', event => {
    const input = event.target;
    setValueAndPreserveSelection(input, toUnicodeFraction(...splitAtSlash(input.value)));
});

function toUnicodeFraction(numerator, denominator) {
    return denominator == null
        ? toSuperscript(numerator)
        : toSuperscript(numerator) + FRACTION_SEPARATOR + toSubscript(denominator);
}

function setValueAndPreserveSelection(input, value) {
    const { selectionStart, selectionEnd } = input;
    Object.assign(input, { value, selectionStart, selectionEnd });
}

function toSuperscript(input) {
    return [...input].map(char => SUPERSCRIPT_MAPPING[char] || char).join('');
}

function toSubscript(input) {
    return [...input].map(char => SUBSCRIPT_MAPPING[char] || char).join('');
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

const FRACTION_SEPARATOR = '⁄';

const SUPERSCRIPT_MAPPING = {
    '-': '⁻',
    '+': '⁺',
    '0': '⁰',
    '1': '¹',
    '2': '²',
    '3': '³',
    '4': '⁴',
    '5': '⁵',
    '6': '⁶',
    '7': '⁷',
    '8': '⁸',
    '9': '⁹',
};

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
