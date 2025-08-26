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
    import { ArrowDownUp, Network, Search, X } from "@lucide/svelte";
    import { onDestroy, onMount } from "svelte";
    import Datepicker from "$lib/DatePicker.svelte";
    import { fade, fly, slide } from "svelte/transition";
    import { quartInOut } from "svelte/easing";

    let tasks: Task[] = $state([]);

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            submitTask();
        }
    }

    async function getIncompleteTasks() {
        tasks = await invoke('get_incomplete_tasks');
    }

    function sortTasksByDueDate(tasks: Task[]): Task[] {
        return [...tasks].sort((a, b) => {
            if (!a.dueDate && !b.dueDate) return 0; // both missing
            if (!a.dueDate) return 1; // a goes last
            if (!b.dueDate) return -1; // b goes last

            // Compare dates
            return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
        });
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
            await invoke('add_database_task', {name: taskName, dueDate: selectedDate?.toISOString(), tags: selectedTags});
            getIncompleteTasks();
            selectedTags = [];
            selectedDate = null;
            taskName = '';
        }
    }

    async function completeTask (taskId: number) {
        await invoke('complete_task', { taskId: taskId });
        getIncompleteTasks();
    }

    async function deleteTask (taskId: number) {
        await invoke("delete_task", {taskId: taskId});
        getIncompleteTasks();
    }

    async function getCompletedTaskCount (): Promise<number> {
        return await invoke<number>('get_completed_task_count');
    }

    function removeTagFromTask(tag: string) {
        selectedTags = selectedTags.filter(t => t !== tag);
    }

    let selectedTags: string[] = $state([]);
    let selectedDate: Date | null = $state(null);



	let taskContainer: HTMLDivElement;
	let header: HTMLHeadingElement;
	let taskBar: HTMLDivElement;

    function getOuterHeight(el: HTMLElement) {
        const style = getComputedStyle(el);
        const marginTop = parseFloat(style.marginTop) || 0;
        const marginBottom = parseFloat(style.marginBottom) || 0;
        return el.offsetHeight + marginTop + marginBottom;
    }

    function resize() {
        if (taskContainer && header && taskBar) {
            const style = getComputedStyle(taskContainer); 
            const marginTop = parseFloat(style.marginTop);
            const marginBottom = parseFloat(style.marginBottom);

            const availableHeight = window.innerHeight 
                - getOuterHeight(header) 
                - getOuterHeight(taskBar)
                - marginTop - marginBottom
                - 180;
            taskContainer.style.height = `${availableHeight}px`;
        }
    }

	onMount(() => {
		requestAnimationFrame(resize);
		window.addEventListener("resize", resize);
	});

	onDestroy(() => {
		window.removeEventListener("resize", resize);
	});

    interface SortOption {
        value: string,
        label: string
    }

    let sortOptions: SortOption[] = [
        { value:"due", label: "due date"},
        { value:"tag", label: "tag"}
    ]
    let sortIndex = $state(0);

    function changeSort() {
        if (sortIndex < sortOptions.length - 1) {
            sortIndex += 1; 
        } else {
            sortIndex = 0;
        }
    }

    let tags = $state([]);

    function dueToday(task: Task) {
        const dueDate: Date | null = task.dueDate ? new Date(task.dueDate) : null;
        const now: Date = new Date();
        if (dueDate?.toLocaleDateString() === now.toLocaleDateString()) {
            return true;
        }
        return false;
    }
</script>


<div class='container'>
    <div class='header'>
        <h1 bind:this={header}>
            Task List
        </h1>
        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
            <h6>
                {tasks.filter(task => dueToday(task)).length} task{tasks.filter(task => dueToday(task)).length !== 1 ? "s" : ''} due today
            </h6>
            <h6>
                {#key tasks}
                    {#await getCompletedTaskCount() then completedTasks}
                        {completedTasks}
                        total tasks completed
                    {/await}
                {/key}
            </h6>
        </div>
    </div>
    <div class='task-container' bind:this={taskContainer}>
        <div class='task-utilities'>
            <div class="sort">
                <Button class="square small" flavor="primary" Icon={ArrowDownUp} onclick={changeSort} />
                <p>
                    Sorting: 
                    {#key sortIndex}
                        <span style="position: absolute; padding-left: 0.25rem" 
                            in:fly={{ y:10, duration: 150, easing: quartInOut }} 
                            out:fly={{ y:-10, duration: 150, easing: quartInOut }}
                        >
                            {sortOptions[sortIndex].label}
                        </span>
                    {/key}
                </p>
            </div>
        </div>
        <div>
            {#key sortOptions[sortIndex].value}
                <div style="position: absolute" 
                    in:fade={{ duration: 150, easing: quartInOut }} 
                    out:fade={{ duration: 150, easing: quartInOut }}
                >
                    {#if sortOptions[sortIndex].value === 'due'}
                        {#each sortTasksByDueDate(tasks) as task (task.id)}
                            <TaskCard {task} onComplete={completeTask} onDelete={deleteTask}/>
                        {/each}
                    {:else if sortOptions[sortIndex].value === 'tag'}
                        {#each tags as tagCat}
                            <span style="text-transform: capitalize">
                                <h4>{tagCat}</h4>
                            </span>
                            {#each sortTasksByDueDate(tasks) as task (task.id)}
                                {#if task.tags?.some(tag => tag === tagCat)}
                                    <TaskCard {task} onComplete={completeTask} onDelete={deleteTask}/>
                                {/if}
                            {/each}
                        {/each}
                    {/if}
                </div>
            {/key}
        </div>
    </div>
    
    <div class="task-bar" bind:this={taskBar}>
        <Card expanded class="short">
            <Textbox bind:value={taskName} onkeydown={(event: KeyboardEvent) => handleKeydown(event)} {placeholders} />
            {#each selectedTags as tag}
                <Badge flavor="outline" noPadding>
                    <span style="padding-left: 0.5rem">
                        {tag}
                    </span>
                    <Button flavor="ghost" class="square xsmall circular" Icon={X}
                        onclick={() => {
                            removeTagFromTask(tag);
                        }}
                    />
                </Badge>
            {/each}
            {#if selectedDate}
                {selectedDate.toLocaleDateString()}
            {/if}
            <TagSelector bind:selectedTags={selectedTags} refreshTags={getIncompleteTasks()} bind:allTags={tags} />
            <Datepicker bind:selectedDate={selectedDate}/>
            <Button onclick={submitTask} class="square" flavor="primary" Icon={ArrowUp} />
        </Card>
    </div>
</div>

<style>
    p {
        font-size: 1rem;
    }

    .header {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .sort {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
    }

    .task-utilities {
        display: flex;
        gap: 0.5rem;
        width: 100%;
    }

    .container {
		width: 100%;
		height: 100%;
        display: flex;
        flex-direction: column;
    }

    .task-container {
        position: relative;
        min-height: 0;
        overflow-y: auto;
        box-shadow: 0px 0px 5px -2px var(--border-color);
        border: 1px solid var(--border-color);
        background-color: var(--primary-light);
        border-radius: 15px;
        padding: 1rem;
        margin-bottom: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
    
</style>