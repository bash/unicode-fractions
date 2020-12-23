import { curryVulgarFraction } from './wrapper.js';

export function bindUserControls({ exports, memory }) {
    const input = document.querySelector('#input');
    const resultElement = document.querySelector('#fraction');
    const copyButton = document.querySelector('#copy');

    const vulgarFraction = curryVulgarFraction(exports, memory);
    const vulgarFractionWithFallback = curryVulgarFractionWithFallback(vulgarFraction);

    function updateOutput() {
        resultElement.value = vulgarFractionWithFallback(input.value);
        updateCopyButtonState(resultElement, copyButton);
    }

    input.addEventListener('input', updateOutput);
    copyButton.addEventListener('click', () => copyToClipboard(resultElement));
}


function inputHasValue(inputElement) {
    return inputElement.value != '';
}

function copyToClipboard(inputElement) {
    const COPY_COMMAND = 'copy';
    if (inputHasValue(inputElement)) {
        inputElement.focus();
        inputElement.select();
        document.execCommand(COPY_COMMAND);
    }
}

function updateCopyButtonState(inputElement, copyButton) {
    copyButton.disabled = !inputHasValue(inputElement);
}

function curryVulgarFractionWithFallback(vulgarFraction) {
    return (input) => {
        const fraction = parseAsciiFraction(input);
        console.log(fraction);
        return (fraction != null)
            ? vulgarFraction(fraction.nominator, fraction.denominator)
            : input;
    }
}

function parseAsciiFraction(input) {
    const parts = input.split('/').map(p => p.trim());
    console.log(input, parts);
    
    if (parts.length !== 2) return null;

    const nominator = Number.parseInt(parts[0]);
    const denominator = Number.parseInt(parts[1]);

    return (!Number.isNaN(nominator) && !Number.isNaN(denominator)
        ? { nominator, denominator }
        : null);
}
