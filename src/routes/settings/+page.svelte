<script lang='ts'>
    import AlertDialog from "$lib/AlertDialog.svelte";
    import Button from "$lib/Button.svelte";
    import Card from "$lib/Card.svelte";
    import Dropdown from "$lib/Dropdown.svelte";
    import Swatch from "$lib/Swatch.svelte";
    import { setColors, type Theme } from "$lib/theme";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from "@tauri-apps/plugin-store";
    import { theme, themes } from "$lib/stores.svelte";
    import { onMount } from "svelte";

    async function resetDatabase() {
        await invoke('reset_database');
        databaseConfirmDialogOpen = false;
        confirmDialog = null;
    }

    async function confirmResetDatabase() {
        databaseConfirmDialogOpen = true;
    }

    $effect(() => {
        if (confirmDialog === "continue") {
            resetDatabase();
        }
    });

    let databaseConfirmDialogOpen = $state(false);
    let confirmDialog = $state();
    
    let selectedTheme: string = $derived("");
     
    async function updateTheme(newThemeName: string) {
        if (newThemeName) {
            theme.theme = themes[newThemeName]
            const store = await load(".settings.json");
            await store.set("theme", { value: themes[newThemeName] });
            await store.save();
            setColors(document.documentElement, themes[newThemeName]);
        }
    }

    $effect(() => {
        if (selectedTheme) {
            updateTheme(selectedTheme);
        }
    });

    const root = document.documentElement;

    onMount(async () => {
        if (theme.theme) {
            selectedTheme = theme.theme.name;
        } else {
            const store = await load(".settings.json");
            const value = await store.get<{ value: Theme }>("theme");

            if (value?.value) {
                theme.theme = value.value;
                selectedTheme = theme.theme.name;
            }
        }
    })


</script>

<h1>
    Settings
</h1>
<div class='stretch'>
    <h5>
        Theme
    </h5>
    <Card expanded>
        <h6>
           Change Theme 
        </h6>
        <p>
            Choose between a preselected theme or make your own!
        </p>
        <Dropdown options={["Pink Light", "Pink Dark"]} bind:selected={selectedTheme}/>
        <p>
            Theme colors
        </p>
        <div class='swatches'>
            <Swatch color={"var(--primary-light)"}/>
            <Swatch color={"var(--primary-dark)"}/>
            <Swatch color={"var(--hover-primary-dark)"}/>
            <Swatch color={"var(--primary-color)"}/>
            <Swatch color={"var(--secondary-color)"}/>
            <Swatch color={"var(--highlight-color)"}/>
            <Swatch color={"var(--border-color)"}/>
            <Swatch color={"var(--hover-color)"}/>
        </div>
    </Card>
</div>
<h5>
    Study
</h5>
<card>

</card>
<div class='stretch'>
    <h5>
        Tasks
    </h5>
    <Card expanded>
        <div class='stretch'>
            <AlertDialog 
                bind:open={databaseConfirmDialogOpen}
                bind:result={confirmDialog}
                title="Are you absolutely sure?"
                message="This action cannot be undone. This will permanently delete both completed and incompleted tasks you've created, as well as any tags."
            />
            <h6>
                Reset Database
            </h6>
            <p>
                This removes <span class='red'>all</span> tasks, including completed and incompleted tasks as well as any created tags!
            </p>
            <div class='reset-database'>
                <Button flavor="danger" class='rounded border' onclick={confirmResetDatabase}>
                    Reset Database
                </Button>
            </div>
        </div>
    </Card>
</div>

<style>
    h6 {
        padding-bottom: 0.5rem;
    }
    p {
        padding-bottom: 1rem;
    }

    .red {
        color: #D64540;
    }

    .reset-database {
        width: 10rem;
    }

    .swatches {
        display: flex;
        border: 1px solid var(--border-color);
        width: fit-content;
    }

</style>