<script lang="ts">
    import { ChevronDown, ChevronUp } from "@lucide/svelte";
    import Button from "./Button.svelte";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    
    let {
        num = $bindable(120),
        upperLimitNum = 995,
        lowerLimitNum = 10,
        increment = 5,
        label = "",
        roundtoNearest = 1
    }: {
        num?: number,
        upperLimitNum?: number,
        lowerLimitNum?: number,
        increment?: number,
        label?: string,
        roundtoNearest?: number

    } = $props();

    let numStr: string = $derived(num.toString());
    
	let inputField: HTMLInputElement;

    
	onMount(() => {
        window.addEventListener("mousedown", handleClickOutside);
		return () => window.removeEventListener("mousedown", handleClickOutside);
	});

    function roundToNearestMultiple (value: number, roundToNearest: number): number {
        return Math.round(value / roundToNearest) * roundtoNearest;
    }

    function setButtonEnabling () {
        incrementButtonEnabled = true;
        decrementButtonEnabled = true;
        if (num + increment > upperLimitNum) {
            incrementButtonEnabled = false;
        } else if (num - increment < lowerLimitNum) {
            decrementButtonEnabled = false;
        }
    }

    function handleClickOutside(event: MouseEvent) {
        if (!inputField.contains(event.target as Node)) {
            const value = +inputField.value;
            if (value > upperLimitNum) {
                num = upperLimitNum;
            } else if (value < lowerLimitNum || !value) {
                num = lowerLimitNum;
            } else {
                num = roundToNearestMultiple(value, roundtoNearest);
            }
            numStr = num.toString();
            setButtonEnabling();
        }
    }
    
    let hasSelectedInput = false;

    let incrementButtonEnabled = $state(true);
    let decrementButtonEnabled = $state(true);


    function handleHoursInput(event: Event) {
        const target = event.target as HTMLInputElement;
        let digits = target.value.replace(/\D/g, '').slice(-3);
        numStr = digits;
    }

    function handleSelect(event: Event): void {
        const target = event.target as HTMLInputElement;
        target.select();
        hasSelectedInput = true;
    }

    function incrementNumber() {
        decrementButtonEnabled = true;
        if (num + increment <= upperLimitNum) {
            num += increment;
            incrementButtonEnabled = true;
        } else {
            incrementButtonEnabled = false;
        }
    }
    function decrementNumber() {
        incrementButtonEnabled = true;
        if (num - increment >= lowerLimitNum) {
            num -= increment;
            decrementButtonEnabled = true;
        } else {
            decrementButtonEnabled = false;
        }
    }

    function createHoldHandler(action: () => void) {
    let holdTimeout: ReturnType<typeof setTimeout> | null = null;
    let repeatInterval: ReturnType<typeof setInterval> | null = null;
    let held = false;

    function handleMouseDown() {
        held = false;

        action(); // immediate call

        holdTimeout = setTimeout(() => {
        held = true;
        repeatInterval = setInterval(() => {
            action();
        }, 20);
        }, 500);
    }

    function handleMouseUp() {
        if (holdTimeout) clearTimeout(holdTimeout);
        if (repeatInterval) clearInterval(repeatInterval);

        holdTimeout = null;
        repeatInterval = null;
        held = false;
    }

    return { handleMouseDown, handleMouseUp };
    }

    const {
        handleMouseDown: startIncrementHold,
        handleMouseUp: stopIncrementHold,
    } = createHoldHandler(incrementNumber);

    const {
        handleMouseDown: startDecrementHold,
        handleMouseUp: stopDecrementHold,
    } = createHoldHandler(decrementNumber);



</script>

<div class="timer-container">
    <div class="countdown-display" >
        <div class="input">
            <input
            onfocus={handleSelect}
            oninput={handleHoursInput}
            type="text"
            maxlength={upperLimitNum.toString().length}
            bind:value={numStr}
            bind:this={inputField}
            />
            <p>{label}</p>
        </div>
        <div class="buttons">
            <Button 
                flavor={incrementButtonEnabled ? "ghost" : "disabled"}
                onmousedown={startIncrementHold}
                onmouseup={stopIncrementHold}
                onmouseleave={stopIncrementHold}
                >
                <ChevronUp />
            </Button>
            <Button
                flavor={decrementButtonEnabled ? "ghost" : "disabled"}
                onmousedown={startDecrementHold}
                onmouseup={stopDecrementHold}
                onmouseleave={stopDecrementHold}
                >
                <ChevronDown />
            </Button>
        </div>
    </div>
</div>

<style>
    p {
        font-family: 'Inter', sans-serif;
        font-size: 0.9rem;
    }
    .input {
        display:flex;
        flex-direction: column;
    }
    
    .buttons {
        display: flex;
        flex-direction: column;
        border-left: 1px solid var(--border-color);
    }
    
    input {
        background-color: var(--primary-light);
        margin: 0;
        padding: 0;
        width: 8.5rem;
        border: none;
        font-family: 'Inter', sans-serif;
        font-size: 3.5rem;
        text-align: center;
    }
    input:focus {
        outline: none;
    }
    .timer-container {
        border-radius: 7px;
        text-align: center;
        font-family: sans-serif;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
    
    .countdown-display {
        box-shadow: 0px 0px 5px -2px var(--border-color);
        border: 1px solid var(--border-color);
        margin: 1rem auto;
        border-radius: 7px;
        display:flex;
        justify-content: center;
        overflow: hidden;
        width:fit-content;
        align-items: center;
    }
</style>
