<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import Card from "$lib/Card.svelte";
    import Badge from "$lib/Badge.svelte";
    import type { Task } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";
    import Textbox from "$lib/Textbox.svelte";
    import Button from "$lib/Button.svelte";
    import ArrowUp from "@lucide/svelte/icons/arrow-up";
    import TagSelector from "$lib/TagSelector.svelte";
    import { X } from "@lucide/svelte";
    import { onMount } from "svelte";
    import Datepicker from "$lib/DatePicker.svelte";

    let tasks: Task[] = $state([]);

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            submitTask();
        }
    }

    async function getIncompleteTasks() {
        tasks = await invoke('get_incomplete_tasks');
    }

    onMount (async () => {
        getIncompleteTasks();
    });

    let placeholders = [
        'steal grandma\'s bagel',
        'read War and Peace',
        'get absolutely wasted on a Tuesday morning',
        'scroll on social media for 6 hours',
        'add an item to my task list',
        'make a sad peanut butter jelly sandwich',
        'cry myself to sleep in a fetal position',
        'sigh heavily and gaze forlornly out the window',
        'talk to myself like a crazy person for an hour',
        'debate with my coffee whether it\'s time to quit or keep going',
        'become a conspiracy theorist',
        'run laps in the swivel chair around the cubicle',
        'make a playlist called \'songs to pretend you\'re working to\'',
        'try drinking a glass of milk while updside down',
        'bring a fishing pole to the aquarium',
        'call John and tell him I can\'t talk right now',
        'drive around in a clown costume like the clown I am',
        'streak through the streets, shouting Eureka!',
        'write a strongly worded email',
        'impersonate a drunk Lyndon B. Johnson looking for his car keys'
    ]

    let taskName = $state("");

    async function submitTask () {
        if (taskName) {
            await invoke('add_database_task', {name: taskName, dueDate: dueDate, tags: selectedTags});
            getIncompleteTasks();
            console.log(tasks);
            selectedTags = [];
            taskName = '';
        }
    }

    async function completeTask (taskId: number) {
        console.log("completing task...");
        await invoke('complete_task', { taskId: taskId });
        getIncompleteTasks();
    }

    function removeTagFromTask(tag: string) {
        selectedTags = selectedTags.filter(t => t !== tag);
    }

    let selectedTags: string[] = $state([]);
    let dueDate: string | null = $state(null);
</script>

<h1>
    Task List
</h1>

<div class='container'>
    <Card expanded>
        {#each tasks as task (task.id)}
            <TaskCard {task} onComplete={completeTask}/>
        {/each}
    </Card>
    
    <Card expanded class="short">
        <Textbox bind:value={taskName} onkeydown={(event: KeyboardEvent) => handleKeydown(event)} {placeholders} />
        {#each selectedTags as tag}
            <Badge flavor="outline" noPadding>
                <span style="padding-left: 0.5rem">
                    {tag}
                </span>
                <Button flavor="ghost" class="square xsmall rounded" Icon={X} 
                    onclick={() => {
                        removeTagFromTask(tag);
                    }}    
                />
            </Badge>
        {/each}
        <TagSelector bind:selectedTags={selectedTags} />
        <Datepicker />
        <Button onclick={submitTask} class="square" flavor="primary" Icon={ArrowUp} />
    </Card>
</div>

<style>
    .container {
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
</style>