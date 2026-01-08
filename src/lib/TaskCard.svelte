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
    import DatePicker from "./DatePicker.svelte";
    import { fly } from "svelte/transition";

    interface Props {
        task: Task;
        allowsEdit?: boolean;
        onComplete: (id: number) => void;
        onDelete?: (id: number) => void;
    }

    let { task = $bindable(), allowsEdit = true, onComplete = $bindable(), onDelete = $bindable() }: Props = $props();

    function complete() {
        onComplete(task.id);
    }

    function deleted() {
        onDelete!(task.id);
    }

    let dueDate: Date | null = $state(task.dueDate ? new Date(task.dueDate) : null);

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
    const dueToday = $derived(dueDate && isSameLocalDate(dueDate, now));
    const overdue = $derived(dueDate && isPastDue(dueDate, now));
    let editing = $state(false);

    let container: HTMLDivElement; // reference to the task container

    $effect(() => {
        if (!editing) return;

        function onClickOutside(event: MouseEvent) {
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

        return () => {
            document.removeEventListener("mousedown", onClickOutside);
            document.removeEventListener("keydown", onKeyDown);
        };
    });

    function commitAndCleanup() {
        commit();
        refreshTask();
    }

    function cancelAndCleanup() {
        cancel();
        refreshTask();
    }


    function startEditing() {
        if (allowsEdit)
            editing = true;
    }


    function commit() {
        editing = false;
    }

    function cancel() {
        editing = false;
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

    let name = $state(task.name);

    // probably unnecessary
    $effect(() => {
        let cancelled = false;

        (async () => {
            await invoke('update_task_name_by_id', { 'taskId': task.id, 'newName': name });
        })();

        return () => {
            cancelled = true;
        };
    });

    $effect(() => {
        (async () => {
            await invoke('update_task_due_date_by_id', { 'taskId': task.id, 'newDueDate': dueDate });
        })();
    });

    async function removeDate() {
        dueDate = null;
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
            {#key task}
                <p class="date">
                    {dueDate ? dueDate.toLocaleDateString(undefined, { month: 'long', day: 'numeric' }) : ''}
                </p>
            {/key}
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
                <Textbox preamble={false} placeholders={["Enter task name"]} bind:value={name} />
            </div>
            {#if dueDate}
                <div 
                    style="display: flex; gap: 0.5rem; align-items: center;"
                    in:fly|global={{ duration: 300, y:7, easing: quartOut }}
                >
                    {dueDate.toLocaleDateString()}
                    <Button class="square xsmall" Icon={X} flavor='outline' onclick={removeDate}/>
                </div>
            {/if}
            <div>
                <DatePicker size="small" slowAnimation={false} bind:selectedDate={dueDate}/>
            </div>
            {#key task}
                {#await getAllTags() then tags}
                    <div style="display: flex; justify-content:flex-start; gap: 0.5rem;">
                        {#each tags as tag (tag.name)}
                            <div 
                                animate:flip={{ duration: 300, easing: quartInOut }}
                            >
                                <div>
                                    {@render tagsn(tag.name, tag.id, tag?.color)}
                                </div>
                            </div>
                        {/each}
                    </div>
                {/await}
            {/key}
            {#if onDelete}
                <div in:fly|global={{ duration: 300, y:7, easing: quartOut }}>
                    <Button onclick={deleted} Icon={Trash} flavor="outline" class="square small" />
                </div>
            {/if}
        {/if}
    </div>
</div>

<style>
    .task-container {
        gap: 1rem;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: space-between;
        width: fit-content;
        padding: 0rem 0.5rem;
        min-height: 3rem;
        max-height: 9rem;
        max-width: 100%;
        box-sizing: border-box;
        border-radius: 15px;
        border: 1px solid transparent;
        transition: 300ms ease-in-out;
    }

    .task-container.edit {
        border: 1px solid var(--border-color);
    }

    .task-card {
        gap: 1rem;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: space-between;
        padding: 0rem 0.5rem;
        width: 100%;
        height: auto;
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
