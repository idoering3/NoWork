<script lang='ts'>
    import Button from '$lib/Button.svelte';
    import Card from '$lib/Card.svelte';
    import Dropdown from '$lib/Dropdown.svelte';
    import NumberInput from '$lib/NumberInput.svelte';
    import Countdown from '$lib/Countdown.svelte';
    import type { StudyType } from '$lib/types/Study.ts';
    import { timerStore } from '$lib/types/timerStore.svelte';
    import { quartOut } from 'svelte/easing';
    import { fly } from 'svelte/transition';
    
    let studyTypes: Record<string, StudyType> = {
        "Pomodoro": {name: "Pomodoro", studyTime: 25, breakTime: 5},
        "Long Pomodoro": {name: "Long Pomodoro", studyTime: 50, breakTime: 10},
        "Deep Work": {name: "Deep Work", studyTime: 90, breakTime: 20},
        "Sprint": {name: "Sprint", studyTime: 15, breakTime: 5}
    }
    let selectedName = $state(studyTypes["Pomodoro"].name);

    let studyTime = $state(studyTypes["Pomodoro"].studyTime);
    let breakTime = $state(studyTypes["Pomodoro"].breakTime);
    let repetitions = $state(5);

    $effect(() => {
        const selected = studyTypes[selectedName];
        if (selected) {
            studyTime = selected.studyTime;
            breakTime = selected.breakTime;
        }
    });

    $effect(() => {
        let foundMatch = false;
        for (const studyType of Object.values(studyTypes)) {
            if (studyType.studyTime === studyTime && studyType.breakTime === breakTime) {
                selectedName = studyType.name;
                foundMatch = true;
            }
        }
        if (!foundMatch) {
            selectedName = '';
        }
    })

    function startTimer() {
        timerStore.init({
            studyTime: studyTime * 60 * 1000,
            breakTime: breakTime * 60 * 1000,
            repetitions: repetitions,
            timerEnabled: true
        });
        timerStore.start();
    }
</script>


<div style="padding: 3rem; display: flex; align-items:center; justify-content: center; flex-direction: column;">
    <h1 in:fly|global={{ y: 30, delay: 150, duration: 1500, easing: quartOut}}>
        Study
    </h1>
    <div class="container">
        {#if timerStore.isEnabled}
            <div class="timer">
                <Countdown />
            </div>
        {:else}
            <div>
                <div transition:fly|global={{ y: 30, delay: 300, duration: 1500, easing: quartOut}}>
                    <Dropdown options={["", ...Object.keys(studyTypes)]} bind:selected={selectedName}></Dropdown>
                </div>
                <div class="side-by-side">
                    <div class="stack" transition:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
                        <h7 class="header">Study</h7>
                        <NumberInput label="mins" roundtoNearest={5} bind:num={studyTime}/>
                    </div>
                    <div class="stack" transition:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
                        <h7 class="header">Break</h7>
                        <NumberInput label="mins" roundtoNearest={1} increment={1} upperLimitNum={60} lowerLimitNum={0} bind:num={breakTime}/>
                    </div>
                </div>
                    <div class="stack" transition:fly|global={{ y: 30, delay: 900, duration: 1500, easing: quartOut}}>
                        <h7 class="header">Repetitions</h7>
                        <NumberInput label="times" roundtoNearest={1} increment={1} upperLimitNum={30} lowerLimitNum={0} bind:num={repetitions}/>
                    </div>
                <div transition:fly|global={{ y: 30, delay: 1200, duration: 1500, easing: quartOut}}> 
                    <Button flavor="primary" onclick={startTimer}>Start Studying</Button>
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        height: 27rem;
        width: 27rem;
    }

    .timer {
        display: flex;
        justify-content: center;
    }

    .header {
        margin-top: 1rem;
        line-height: 0.5rem;
    }

    .stack {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
    .side-by-side {
        gap: 1rem;
        display: flex;
        vertical-align: middle;
        justify-content: center;
        align-items: center;
    }
</style>