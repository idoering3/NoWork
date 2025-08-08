<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import Card from "$lib/Card.svelte";
    import type { Task } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";
    import Textbox from "$lib/Textbox.svelte";
    import Button from "$lib/Button.svelte";
    import ArrowUp from "@lucide/svelte/icons/arrow-up";

    let tasks: Task[] = [
        {
            id: 1,
            name: "Finish NoWork UI",
            dueDate: "2025-08-10T12:00:00Z",
            createdAt: "2025-08-06T10:00:00Z",
            completed: false,
            completedAt: null,
            tags: "work"
        },
        {
            id: 2,
            name: "Review async Rust",
            dueDate: null,
            createdAt: "2025-08-04T14:00:00Z",
            completed: true,
            completedAt: "2025-08-05T15:00:00Z",
            tags: "study"
        }
    ];

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
        'make a playlist called \'songs to pretend you\'re working to\''
    ]

</script>

<h1>
    Task List
</h1>

<div class='container'>
    <Card expanded>
        {#each tasks as task (task.id)}
            <TaskCard {task} />
        {/each}
    </Card>
    
    <Card expanded class="short">
        <Textbox placeholders={placeholders}/>
        <Button class="square" Icon={ArrowUp}/>
    </Card>
</div>

<style>
    .container {
        overflow-y: auto;
        height: 100vh;
    }
</style>