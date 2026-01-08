<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import Card from "$lib/Card.svelte";
    import Badge from "$lib/Badge.svelte";
    import type { Task, Tag } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";
    import Textbox from "$lib/Textbox.svelte";
    import Button from "$lib/Button.svelte";
    import ArrowUp from "@lucide/svelte/icons/arrow-up";
    import TagSelector from "$lib/TagSelector.svelte";
    import { ArrowDownUp, Network, Plus, Search, X } from "@lucide/svelte";
    import { onDestroy, onMount, tick } from "svelte";
    import Datepicker from "$lib/DatePicker.svelte";
    import { fade, fly, scale, slide } from "svelte/transition";
    import { circInOut, quartIn, quartInOut, quartOut } from "svelte/easing";
    import { load, Store } from "@tauri-apps/plugin-store";
    import { flip } from "svelte/animate";

    let tasks: Task[] = $state([]);
    let show = $state(false);


    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault();
            event.stopPropagation();
            submitTask();
        }
    }


    async function getAllTags() {
        tags = await invoke<Tag[]>('get_all_tags'); 
    }

    async function getIncompleteTasks() {
        tasks = await invoke('get_incomplete_tasks');
        completedTasks = await getCompletedTaskCount();
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
        'impersonate a drunk Lyndon B. Johnson looking for his car keys',
        'delete those incriminating Watergate tapes',
        'go for a run',
        'pretend to do your work'
    ]

    let taskName = $state("");

    async function submitTask () {
        if (taskName) {
            await invoke('add_database_task', {name: taskName, dueDate: selectedDate?.toISOString(), tags: selectedTags});
            getIncompleteTasks();
            selectedDate = null;
            taskName = '';
            if (selectedTag) {
                selectedTags = [selectedTag];
            }
        }
    }

    async function completeTask (taskId: number) {
        await invoke('complete_task', { taskId: taskId });
        await getIncompleteTasks();
    }

    async function deleteTask (taskId: number) {
        await invoke("delete_task", {taskId: taskId});
        await getIncompleteTasks();
    }

    async function getCompletedTaskCount (): Promise<number> {
        return await invoke<number>('get_completed_task_count');
    }

    function removeTagFromTask(tag: string) {
        selectedTags = selectedTags.filter(t => t.name !== tag);
    }

    let selectedTags: Tag[] = $state([]);
    let selectedDate: Date | null = $state(null);



	let taskContainer: HTMLDivElement;
	let header: HTMLHeadingElement;
	let taskBar: HTMLDivElement | undefined = $state();

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

    let runCollapse = $state(true);

	onMount(() => {
		requestAnimationFrame(resize);
		window.addEventListener("resize", resize);
        window.addEventListener("keydown", handleKeydown);
        show = true;
	});

	onDestroy(() => {
		window.removeEventListener("resize", resize);
		window.removeEventListener("keydown", handleKeydown);
	});

    let tags: Tag[] = $state([]);

    function dueToday(task: Task) {
        const dueDate: Date | null = task.dueDate ? new Date(task.dueDate) : null;
        const now: Date = new Date();
        if (dueDate?.toLocaleDateString() === now.toLocaleDateString()) {
            return true;
        }
        return false;
    }

    //TODO better filterMode
    let filterMode: "all" | "tag" = $state("tag");
    let selectedTag: Tag | null = $state(null);

    $effect(() => {
        if (filterMode === "tag" && selectedTag) {
            selectedTags = [selectedTag];
        } else {
            selectedTags = [];
        }
    });

    onMount (async () => {
        getIncompleteTasks();
        getAllTags();
        const store = await load(".settings.json");

        let filterModeVar = await store.get<"all" | "tag">("filterMode");
        if (filterModeVar) {
            filterMode = filterModeVar;
        } else {
            filterMode = "all";
        }
        const tag = await store.get<{ id: number, name: string, color: 'default' | 'outline' | 'danger' | 'blue' }>("selectedTag");
        if (tag) {
            selectedTag = tag;
        }
        completedTasks = await getCompletedTaskCount();
    });

    let completedTasks = $state();

    async function selectTag(tag: Tag) {
        await getIncompleteTasks();
        runCollapse = true;
        filterMode = "tag";
        selectedTag = tag;

        const store = await load(".settings.json");
        await store.set("filterMode", "tag");
        await store.set("selectedTag", tag);
    }

    async function selectAllTasks() {
        await getIncompleteTasks();
        runCollapse = true;
        filterMode = "all";
        selectedTag = null;

        const store = await load(".settings.json");
        await store.set("filterMode", "all");
        await store.delete("selectedTag");
    }

    let visibleTasks = $derived(tasks
    .filter(task => filterMode === 'all' || task.tags?.some(tag => tag.name === selectedTag?.name))
    .sort((a, b) => {
        if (!a.dueDate && !b.dueDate) return 0;
        if (!a.dueDate) return 1;
        if (!b.dueDate) return -1;
        return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
    }));
</script>

<div style="overflow: hidden; display: flex; height: calc(100vh - 3rem);">
    <div class='sidebar'>
        <p 
            style="padding: 1rem; display:flex; align-items: center; justify-content: center; border-bottom: 1px solid var(--border-color)"
            transition:fly={{ y: 30, delay: 300, duration: 1500, easing: quartOut}}
        >
            Tasks
        </p>
        <div style={selectedTag?.name === "all" ? "" : ""} transition:fly={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
            <Button flavor="ghost" onclick={async () => await selectAllTasks()}><span style={filterMode === "all" ? "color:var(--highlight-color)" : ""}>all tasks</span></Button>
        </div>
        {#each tags as tag, i}
            <div style={tag.name === selectedTag?.name ? "" : ""} transition:fly={{ y: 30, delay: 600 + (i + 1) * 300, duration: 1500, easing: quartOut}}>
                <Button flavor="ghost" onclick={async () => await selectTag(tag)}><span style={tag.name === selectedTag?.name ? "color:var(--highlight-color)" : ""}>{tag?.name}</span></Button>
            </div>
        {/each}
        <!-- <Button flavor="ghost" Icon={Plus}></Button> -->
    </div>
    <div class='container'>
        <div class='header'>
            <h1 bind:this={header} in:fly|global={{ y: 30, delay: 150, duration: 1500, easing: quartOut}}>
                Task List
            </h1>
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <h6 transition:fly={{ x: -15, delay: 600, duration: 1500, easing: quartOut}}>
                    {tasks.filter(task => dueToday(task)).length} task{tasks.filter(task => dueToday(task)).length !== 1 ? "s" : ''} due today
                </h6>
                <h6 transition:fly={{ x: -15, delay: 1200, duration: 1500, easing: quartOut}}>
                    {completedTasks} total tasks completed
                </h6>
            </div>
        </div>
        <div class='task-container' bind:this={taskContainer} transition:fly|global={{ duration: 1500, delay:300, y:30, easing: quartOut }}>
            <div style="position: relative;">
                {#key selectedTag}
                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                        {#each visibleTasks as task, i (task.id)}
                        <!-- No effing clue why, but the animate and transitions MUST be separated. It breaks otherwise -->
                            <div animate:flip|global={{ duration: 300, easing: quartInOut }}>
                                <div
                                    in:fly|global={{ duration: 1000, y: 15, easing: quartOut, delay: runCollapse ? 150 + 75 * (i + 1) : 0 }}
                                    out:fly|global={{ duration: 150, y: -15, easing: quartIn }}
                                    onintroend={() => runCollapse ? runCollapse = false : ""}
                                >
                                    <TaskCard {task} onComplete={completeTask} onDelete={deleteTask}/>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/key}
            </div>
        </div>
        {#if show}
            <div class="task-bar" bind:this={taskBar} transition:fly|global={{ duration: 1500, delay:600, y:30, easing: quartOut }}>
                <Card expanded class="short">
                    <Textbox bind:value={taskName} {placeholders} />
                    {#snippet tagsn(name: string, color: 'default' | 'outline' | 'danger' | 'blue')}
                        <Badge flavor={color} noPadding>
                            <span style="padding-left: 0.5rem">
                                {name}
                            </span>
                            <Button flavor="badge" class="square xsmall circular" Icon={X}
                                onclick={() => {
                                    removeTagFromTask(name);
                                }}
                            />
                        </Badge>
                    {/snippet}
                    {#key selectedTag?.name}
                        <div
                            style="display: flex;"
                        >
                        {#each selectedTags as tag (tag.name)}
                            <div animate:flip|global={{ duration: 300, easing: quartInOut }} style="padding: 0.25rem;">
                                <div style=""
                                >
                                    {@render tagsn(tag.name, tag?.color)}
                                </div>
                            </div>
                        {/each}
                        </div>
                    {/key}
                    {#if selectedDate}
                        {selectedDate.toLocaleDateString()}
                    {/if}
                    <TagSelector bind:selectedTags={selectedTags} refreshTags={getAllTags} bind:allTags={tags} />
                    <Datepicker bind:selectedDate={selectedDate}/>
                    <div transition:fly|global={{ duration: 1500, delay:1200, y:7, easing: quartOut }}>
                        <Button onclick={submitTask} class="square" flavor="primary" Icon={ArrowUp} />
                    </div>
                </Card>
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        padding: 1rem 3rem;
        overflow: hidden;
    }

    .sidebar {
        width: 15rem;
        border-right: 1px solid var(--border-color);
        height: calc(100vh - 3rem);
    }

    p {
        font-size: 1rem;
    }

    .header {
        display: flex;
        align-items: center;
        gap: 1rem;
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