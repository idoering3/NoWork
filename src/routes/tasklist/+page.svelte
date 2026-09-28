<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import { type Task, type Tag, type TaskPriority, getCompletedTaskCount, submitTask, getIncompleteTasks, type CreateTask, completeTask, deleteTask } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";
    import { onDestroy, onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { quartIn, quartInOut, quartOut } from "svelte/easing";
    import { load } from "@tauri-apps/plugin-store";
    import { flip } from "svelte/animate";
    import CustomScrollbar from "$lib/misc/CustomScrollbar.svelte";
    import { setPageEl } from "$lib/misc/context";
    import { matchesFilter, type TaskFilter } from "$lib/types/filter";
    import FilterBar from "$lib/FilterBar.svelte";
    import TaskInput from "$lib/input-fields/TaskInput.svelte";
    import { getAllTags } from "$lib/stores.svelte";
    import { completedTaskCount, taskState } from "$lib/types/taskStore.svelte";

    let show = $state(false);

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault();
            event.stopPropagation();
        }
    }

    // TODO also move to task.ts
    async function refreshTask(taskId: number) {
        const updatedTask = await invoke<Task>('get_task_by_id', { 'taskId':taskId });

        taskState.tasks = taskState.tasks.map(task => task.id === updatedTask.id ? updatedTask : task);
    }

    // the filtering variable that stores all filters
    let filter = $state<TaskFilter>({
        tags: [],
        priorities: [],
        date: null,
    });


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

    let selectedTag: Tag | null = $state(null);

    onMount (async () => {
        taskState.tasks = await getIncompleteTasks();
        tags = await getAllTags();
        const store = await load(".settings.json");

        let filterStore = await store.get<TaskFilter>("taskFilter");
        if (filterStore) {
            filter = filterStore;
            console.log(filter);
        }

        const tag = await store.get<{ id: number, name: string, color: 'default' | 'outline' | 'danger' | 'blue' }>("selectedTag");
        if (tag) {
            selectedTag = tag;
        }
    });

    let visibleTasks = $derived(
        taskState.tasks
            .filter(task => matchesFilter(task, filter))
            .sort((a, b) => {
                if (!a.dueDate && !b.dueDate) return 0;
                if (!a.dueDate) return 1;
                if (!b.dueDate) return -1;
                return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
    }));

    let pageEl = $state<HTMLElement>();
    setPageEl( () => pageEl );

    async function saveFilter(filter: TaskFilter) {
        const store = await load(".settings.json");
        await store.set("taskFilter", filter);
        await store.save();
    }
</script>









<div style="overflow: hidden; display: flex; height: calc(100vh - 3rem);" bind:this={pageEl}>
    <div class='container'>
        <!-- TOP HEADER WITH TASKS AND INFO -->
        <div class='header'>
            <h1 bind:this={header} in:fly|global={{ y: 30, delay: 150, duration: 1500, easing: quartOut}}>
                Tasks
            </h1>
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <h6 in:fly={{ x: -15, delay: 600, duration: 1500, easing: quartOut}}>
                    {taskState.tasks.filter(task => dueToday(task)).length} task{taskState.tasks.filter(task => dueToday(task)).length !== 1 ? "s" : ''} due today
                </h6>
                <h6 in:fly={{ x: -15, delay: 1200, duration: 1500, easing: quartOut}}>
                    {completedTaskCount.completed} total tasks completed
                </h6>
            </div>
        </div>

        <!-- FILTERING GOES HERE???? -->
        <FilterBar bind:filter tags={tags} saveFilter={saveFilter} />

        <div class='task-container' bind:this={taskContainer} in:fly|global={{ duration: 1500, delay:300, y:30, easing: quartOut }}>
            <CustomScrollbar>
                <div style="position: relative;">
                    {#key selectedTag}
                        <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                            {#each visibleTasks as task, i (task.id)}
                            <!-- No effing clue why, but the animate and transitions MUST be separated. It breaks otherwise -->
                                <div animate:flip|global={{ duration: 300, easing: quartInOut }}>
                                    <div
                                            in:fly|global={{ duration: 1000, y: 15, easing: quartOut, delay: runCollapse ? 150 + 75 * (i + 1) : 0 }}
                                            out:fly|global={{ duration: 150, y: -15, easing: quartIn }}
                                            onintroend={() => runCollapse ? runCollapse = false : ""}
                                        >
                                        <TaskCard {task} onComplete={completeTask} onDelete={deleteTask} onUpdate={refreshTask}/>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    {/key}
                </div>
            </CustomScrollbar>
        </div>
        <TaskInput {show} {taskBar}/>
    </div>
</div>

<style>
    .container {
        padding: 1rem 3rem;
        overflow: hidden;
    }

    .header {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .container {
        display: flex;
        width: 100%;
        margin-bottom: 1.5rem;
        flex-direction: column;
    }

    .task-container {
        overflow-y:auto;
        position: relative;
        min-height: 0;
        box-shadow: 0px 0px 5px -2px var(--border-color);
        border: 1px solid var(--border-color);
        background-color: var(--primary-light);
        border-radius: 15px;
        padding: 1rem;
        margin-bottom: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        flex-grow: 1;
    }

</style>