import { curryVulgarFraction } from './wrapper.js';

export function bindUserControls({ exports, memory }) {
    const nominatorInput = document.querySelector('#nominator');
    const denominatorInput = document.querySelector('#denominator');
    const resultElement = document.querySelector('#fraction');
    const copyButton = document.querySelector('#copy');

    const vulgarFraction = curryVulgarFraction(exports, memory);
    const vulgarFractionWithFallback = curryVulgarFractionWithFallback(vulgarFraction);

    function updateOutput() {
        resultElement.value = vulgarFractionWithFallback(
            nominatorInput.valueAsNumber,
            denominatorInput.valueAsNumber);
        updateCopyButtonState(resultElement, copyButton);
    }

    nominatorInput.addEventListener('input', updateOutput);
    denominatorInput.addEventListener('input', updateOutput);
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
    return (nominator, denominator) => {
        const isNominatorValid = Number.isFinite(nominator);
        const isDenominatorValid = Number.isFinite(denominator);
        return isNominatorValid && isDenominatorValid
            ? vulgarFraction(nominator, denominator)
            : '';
    }
}
