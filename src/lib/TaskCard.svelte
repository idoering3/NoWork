<script lang="ts">
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import { Plus, Trash, X } from "@lucide/svelte";
    import Button from "./Button.svelte";
    import Badge from "./Badge.svelte";
    import { quartInOut, quartOut } from "svelte/easing";
    import { onDestroy } from "svelte";
    import { getAllTags } from "./stores.svelte";
    import { flip } from "svelte/animate";
    import Textbox from "./Textbox.svelte";
    import { invoke } from "@tauri-apps/api/core";

    interface Props {
        task: Task;
        onComplete: (id: number) => void;
        onDelete?: (id: number) => void;
    }

    let { task = $bindable(), onComplete = $bindable(), onDelete = $bindable() }: Props = $props();

    function complete() {
        onComplete(task.id);
    }

    function deleted() {
        onDelete!(task.id);
    }

    const dueDate: Date | null = task.dueDate ? new Date(task.dueDate) : null;

    // Local date comparison helpers
    function isSameLocalDate(a: Date, b: Date): boolean {
        return (
            a.getFullYear() === b.getFullYear() &&
            a.getMonth() === b.getMonth() &&
            a.getDate() === b.getDate()
        );
    }

    function isPastDue(a: Date, b: Date): boolean {
        const today = new Date(b.getFullYear(), b.getMonth(), b.getDate());
        const check = new Date(a.getFullYear(), a.getMonth(), a.getDate());
        return check < today;
    }

    const now = new Date();
    const dueToday = dueDate && isSameLocalDate(dueDate, now);
    const overdue = dueDate && isPastDue(dueDate, now);
    let editing = $state(false);

    let container: HTMLDivElement; // reference to the task container

    function startEditing() {
        editing = true;

        function onClickOutside(event: MouseEvent) {
            // Check if click is outside the container
            if (!container.contains(event.target as Node)) {
                commitAndCleanup();
            }
        }

        function onKeyDown(event: KeyboardEvent) {
            if (event.key === "Enter") commitAndCleanup();
            if (event.key === "Escape") cancelAndCleanup();
        }

        document.addEventListener("mousedown", onClickOutside);
        document.addEventListener("keydown", onKeyDown);

        function cleanup() {
            document.removeEventListener("mousedown", onClickOutside);
            document.removeEventListener("keydown", onKeyDown);
        }

        function commitAndCleanup() {
            commit();
            cleanup();
        }

        function cancelAndCleanup() {
            cancel();
            cleanup();
        }

        onDestroy(cleanup); // remove listeners if component unmounts
    }

    function commit() {
        editing = false;
        console.log("committed!");
    }

    function cancel() {
        editing = false;
        console.log("canceled!");
    }

    function onRightClick(event: MouseEvent) {
        event.preventDefault();
        startEditing();
    }

    async function refreshTask() {
        task = await invoke<Task>('get_task_by_id', { 'taskId':task.id });
    }

    async function addTagToTask(id: number) {
        await invoke('add_tag_to_task', { 'taskId':task.id , 'tagId': id });
        await refreshTask();
    }
    
    async function removeTagFromTask(id: number) {
        await invoke('remove_tag_from_task', { 'taskId':task.id , 'tagId': id });
        await refreshTask();
    }
</script>

{#snippet tagsn(name: string, id: number, color: 'default' | 'outline' | 'danger' | 'blue')}
    <Badge flavor={color} noPadding>
        {#if task.tags?.some(tag => tag.name === name)}
            <span style="padding-left: 0.5rem;">
                {name}
            </span>
            <Button flavor="badge" class="square xsmall circular" Icon={X}
                onclick={() => {
                    removeTagFromTask(id);
                }}
            />
        {:else}
            <Button
                flavor="badge"
                class="square xsmall circular"
                Icon={Plus}
                onclick={(event) => {
                    event.stopPropagation();
                    addTagToTask(id);
                }}
            />
            <span style="padding-right: 0.5rem;">
                {name}
            </span>
        {/if}
    </Badge>
{/snippet}

<div
    class="task-container"
    bind:this={container}
    class:edit={editing}
    oncontextmenu={onRightClick}
    role="document"
>
    <div class="task-card" class:overdue={overdue} class:due-today={dueToday}>
        <Button onclick={complete} Icon={Check} flavor="outline" class="square small" />
        {#if !editing}
            <div class="stacked">
                <p style="font-size: 1rem">{task.name}</p>
                <p class="date">
                    {dueDate ? dueDate.toLocaleDateString(undefined, { month: 'long', day: 'numeric' }) : ''}
                </p>
            </div>
            <div class="tags">
                {#if task.tags}
                    {#each task.tags as tag}
                        <Badge flavor={tag.color}>{tag.name}</Badge>
                    {/each}
                {/if}
            </div>
        {:else}
            <div style="width: fit-content;">
                <Textbox preamble={false} placeholders={["Enter task name"]} value={task.name} />
            </div>
            {#key task}
                {#await getAllTags() then tags}
                    {#each tags as tag (tag.name)}
                        <div animate:flip={{ duration: 300, easing: quartInOut }}>
                            <div>
                                {@render tagsn(tag.name, tag.id, tag?.color)}
                            </div>
                        </div>
                    {/each}
                {/await}
            {/key}
        {/if}
    </div>
    {#if onDelete}
        <Button onclick={deleted} Icon={Trash} flavor="outline" class="square small" />
    {/if}
</div>

<style>
    .task-container {
        gap: 1rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: fit-content;
        padding: 0rem 0.5rem;
        height: 3rem;
        min-height: 0;
        box-sizing: border-box;
        border-radius: 15px;
        border: 1px solid transparent;
        overflow:hidden;
        transition: 300ms ease-in-out;
    }

    .task-container.edit {
        border: 1px solid var(--border-color);
    }

    .task-card {
        gap: 1rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: fit-content;
        padding: 0rem 0.5rem;
        height: 3rem;
        min-height: 0;
        border-radius: 0.5rem;
        transition: background-color 0.3s ease, color 0.3s ease;
    }

    .task-card.due-today .date{
        color: #86231c;
    }

    .task-card.overdue .date {
        color: #86231c;
    }

    .date {
        font-size: 0.8rem;
    }

    .stacked {
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .tags {
        display: flex;
        gap: 0.5rem;
    }
</style>
