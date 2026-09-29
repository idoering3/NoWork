<script lang='ts'>
  import Badge from "$lib/Badge.svelte";
  import Button from "$lib/Button.svelte";
  import Card from "$lib/Card.svelte";
  import DatePicker from "$lib/dropdowns/DatePicker.svelte";
  import PrioritySelector from "$lib/dropdowns/PrioritySelector.svelte";
  import TagSelector from "$lib/dropdowns/TagSelector.svelte";
  import { quartInOut, quartOut } from "svelte/easing";
  import Textbox from "./Textbox.svelte";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { ArrowUp, X } from "@lucide/svelte";
  import { submitTask, type CreateTask } from "$lib/tasks/task";
  import { onDestroy, onMount } from "svelte";
  import { parseTaskText, proposeModifications, type TaskModification } from "$lib/tasks/taskParser";
  import TaskTextbox from "./TaskTextbox.svelte";


    interface Props {
        show: boolean;
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

    let proposedTask = $state<CreateTask>({
        name: "",
        dueDate: null,
        tags: [],
        priority: null
    });

    let { show }: Props = $props();

    async function removeDate() {
        proposedTask.dueDate = null;
    }

    function removeTagFromTask(tag: string) {
        proposedTask.tags = proposedTask.tags.filter(t => t.name !== tag);
    }

    async function trySubmitTask() {
        // we need to set the proposed task name to the actual stripped out name
        submitTask(proposedTask);
        resetProposedTask();
    }

    function resetProposedTask() {
        proposedTask = {
            name: "",
            dueDate: null,
            tags: [],
            priority: null
        };
        taskName = "";
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault();
            event.stopPropagation();

            trySubmitTask();
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
   });

   // this runs every time taskName changes
    let modifications = $derived.by<TaskModification[]>(() => parseTaskText(taskName));
    let taskName = $state("");

    let parsedTask = $derived(
        proposeModifications(taskName, modifications)
    );

    $effect(() => {
        if (parsedTask !== null || parsedTask !== undefined) {
            proposedTask.name = parsedTask.name;
            proposedTask.tags = parsedTask.tags;
            proposedTask.priority = parsedTask.priority;
            proposedTask.dueDate = parsedTask.dueDate;
        }
    })
</script>

{#if show}
    <div class="task-bar" in:fly|global={{ duration: 1500, delay:600, y:15, easing: quartOut }}>
        <Card expanded class="short">
            <TaskTextbox bind:value={taskName} {placeholders} {modifications}>
                {taskName}
            </TaskTextbox>
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
            {#key proposedTask.tags.length}
                <div
                    style="display: flex;"
                >
                {#each proposedTask.tags as tag (tag.name)}
                    <div animate:flip|global={{ duration: 300, easing: quartInOut }} style="padding: 0.25rem;">
                        <div style=""
                        >
                            {@render tagsn(tag.name, tag?.color)}
                        </div>
                    </div>
                {/each}
                </div>
            {/key}
            {#if proposedTask.dueDate !== null}
                <div>
                    {proposedTask.dueDate.toLocaleDateString()}
                    <Button class="square xsmall circular" Icon={X} flavor='outline' onclick={removeDate}/>
                </div>
            {/if}
            <PrioritySelector bind:priority={proposedTask.priority}/>
            <TagSelector bind:selectedTags={proposedTask.tags} />
            <DatePicker bind:selectedDate={proposedTask.dueDate}/>
            <div in:fly|global={{ duration: 1500, delay:800, y:5, easing: quartOut }}>
                <Button onclick={trySubmitTask} class="square circular" flavor="primary" Icon={ArrowUp} />
            </div>
        </Card>
    </div>
{/if}