<script lang='ts'>
    import AlertDialog from "$lib/AlertDialog.svelte";
    import Button from "$lib/Button.svelte";
    import Dropdown from "$lib/Dropdown.svelte";
    import Swatch from "$lib/Swatch.svelte";
    import { setColors, type Theme } from "$lib/theme";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from "@tauri-apps/plugin-store";
    import { selectedDateFormat, theme, themes, username } from "$lib/stores.svelte";
    import { onMount } from "svelte";
    import Textbox from "$lib/Textbox.svelte";
    import { fly } from "svelte/transition";
    import { quartOut } from "svelte/easing";
    import { dateFormatOptions, type DateFormatName } from "$lib/misc/datePrints";
    import { loadCredentials, saveCredentials } from "$lib/cal/calendarCredentialStorage";

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
    let calNumHours = $state(10);
    let calStartTime = $state(7);
    let loaded = $state(false);
    let mounted = $state(false);
    let email = $state("");
    let password = $state("");
    
    let selectedTheme = $state("");
    const themeOptions = Object.entries(themes).map(([key, theme]) => ({
        value: key,
        label: theme.name ?? key
    }));
    
    async function updateTheme(newThemeName: string) {
        if (newThemeName) {
            theme.theme = themes[newThemeName]
            const store = await load(".settings.json");
            await store.set("theme", { value: themes[newThemeName] });
            await store.save();
        }
    }

    async function updateDateFormat(newDateFormatFunctionName: DateFormatName) {
        const store = await load(".settings.json");
        await store.set("dateFormat", { value: newDateFormatFunctionName });
        await store.save();
    }

    $effect(() => {
        if (!mounted) return;
        if (selectedTheme) {
            updateTheme(selectedTheme);
        }
    });

    $effect(() => {
        if (!mounted) return;
        if (selectedDateFormat.name) {
            updateDateFormat(selectedDateFormat.name);
        }
    });

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

        const store = await load(".settings.json");

        const dateFormat = await store.get<{ value: DateFormatName }>("dateFormat");
        const calendarNumberHours = await store.get<{ value: number}>("calendarNumHours");
        const calendarStartTime = await store.get<{ value: number}>("calendarStartTime");
        const emailName = await store.get<{ value: string}>("email");

        calNumHours = calendarNumberHours?.value ?? 10;
        calStartTime = calendarStartTime?.value ?? 10;

        if (emailName?.value) {
            email = emailName.value;
            password = await loadCredentials(email);
        }

        selectedDateFormat.name = dateFormat?.value ?? "dayOfWeekAndMonth";
        loaded = true;
        mounted = true;
    });

    $effect(() => {
        if (!mounted) return;
        if (username.name) {
            updateName(username.name);
        }
    });

    async function updateName(newName: string) {
        const store = await load(".settings.json");
        await store.set("username", { value: newName });
        await store.save();
    }

    $effect(() => {
        if (!mounted) return;
        if (calNumHours && loaded) updateCalendarNumHours(calNumHours);
        if (calStartTime && loaded) updateCalendarStartTime(calStartTime);
    });

    async function updateCalendarStartTime(startHour: number) {
        const store = await load(".settings.json");
        await store.set("calendarStartTime", { value: startHour });
        await store.save();
    }

    async function updateCalendarNumHours(hours: number) {
        const store = await load(".settings.json");
        await store.set("calendarNumHours", { value: hours });
        await store.save();
    }

    export async function removeCredentials(email: string): Promise<void> {
        await invoke("delete_credentials", { email });
        const store = await load(".settings.json");
        await store.set("email", { value: "" });
        await store.save();
        password = "";
        email = "";
    }
</script>

<div class="page">
    <h1 in:fly={{ y: 20, delay: 100, duration: 800, easing: quartOut }}>
        Settings
    </h1>

    <div class="settings-grid">

        <!-- Appearance -->
        <section class="card" in:fly={{ y: 20, delay: 200, duration: 800, easing: quartOut }}>
            <h5 class="card-title">Appearance</h5>

            <div class="field">
                <label class="field-label">Display Name</label>
                <p class="field-hint">Shown on the homepage greeting.</p>
                <div class="input-wrap">
                    <Textbox placeholders={["Enter your name"]} preamble={false} bind:value={username.name} />
                </div>
            </div>

            <div class="divider"></div>

            <div class="field">
                <label class="field-label">Theme</label>
                <p class="field-hint">Choose a preset or build your own.</p>
                <Dropdown options={themeOptions} bind:selected={selectedTheme} />
                <div class="swatches">
                    <Swatch color={"var(--primary-light)"} />
                    <Swatch color={"var(--primary-dark)"} />
                    <Swatch color={"var(--hover-primary-dark)"} />
                    <Swatch color={"var(--primary-color)"} />
                    <Swatch color={"var(--secondary-color)"} />
                    <Swatch color={"var(--highlight-color)"} />
                    <Swatch color={"var(--border-color)"} />
                    <Swatch color={"var(--hover-color)"} />
                </div>
            </div>

            <div class="divider"></div>

            <div class="field">
                <label class="field-label">Date Format</label>
                <p class="field-hint">Controls how dates appear throughout the app.</p>
                <Dropdown options={dateFormatOptions} bind:selected={selectedDateFormat.name} />
            </div>
        </section>

        <!-- Calendar -->
        <section class="card" in:fly={{ y: 20, delay: 350, duration: 800, easing: quartOut }}>
            <h5 class="card-title">Calendar</h5>

            <div class="field">
                <label class="field-label">iCloud Account</label>
                <p class="field-hint">Use an app-specific password from appleid.apple.com.</p>
                <div class="input-wrap">
                    <Textbox placeholders={["Email"]} preamble={false} bind:value={email} />
                </div>
                <div class="input-wrap">
                    <Textbox placeholders={["App-specific password"]} preamble={false} bind:value={password} />
                </div>
                <div class="button-row">
                    <Button flavor="primary" class="circular" onclick={() => saveCredentials(email, password)}>
                        Save
                    </Button>
                    <Button flavor="danger" class="circular border" onclick={() => removeCredentials(email)}>
                        Clear
                    </Button>
                </div>
            </div>
        </section>

        <!-- Danger Zone -->
        <section class="card danger-card" in:fly={{ y: 20, delay: 500, duration: 800, easing: quartOut }}>
            <h5 class="card-title">Danger Zone</h5>

            <div class="field">
                <label class="field-label">Reset Database</label>
                <p class="field-hint">
                    Permanently deletes <strong>all</strong> tasks (completed and incomplete) and all tags. This cannot be undone.
                </p>
                <Button flavor="danger" class="rounded border" onclick={confirmResetDatabase}>
                    Reset Database
                </Button>
            </div>
        </section>

    </div>
</div>

<AlertDialog
    bind:open={databaseConfirmDialogOpen}
    bind:result={confirmDialog}
    title="Are you absolutely sure?"
    message="This action cannot be undone. This will permanently delete both completed and incompleted tasks you've created, as well as any tags."
/>

<style>
    .page {
        padding: 3rem;
        max-width: 860px;
    }

    h1 {
        margin-bottom: 2rem;
    }

    /* Grid: two columns on wide viewports, single column otherwise */
    .settings-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
        gap: 1.5rem;
        align-items: start;
    }

    /* Cards */
    .card {
        border: 1px solid var(--border-color);
        border-radius: 15px;
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 0;
    }

    .card-title {
        margin-bottom: 1.25rem;
    }

    .danger-card {
        border-color: color-mix(in srgb, #D64540 40%, var(--border-color));
    }

    /* Fields */
    .field {
        display: flex;
        flex-direction: column;
        gap: 0.375rem;
    }

    .field-label {
        font-weight: 600;
        font-size: 0.875rem;
    }

    .field-hint {
        font-size: 0.8125rem;
        opacity: 0.6;
        margin: 0;
        padding: 0;
        padding-bottom: 0.5rem;
    }

    .divider {
        height: 1px;
        background: var(--border-color);
        margin: 1.25rem 0;
    }

    /* Inputs */
    .input-wrap {
        border: 1px solid var(--border-color);
        border-radius: 15px;
        margin-bottom: 0.5rem;
    }

    /* Buttons */
    .button-row {
        display: flex;
        gap: 0.5rem;
        margin-top: 0.25rem;
    }

    /* Swatches */
    .swatches {
        display: flex;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        overflow: hidden;
        width: fit-content;
        margin-top: 0.75rem;
    }
</style>