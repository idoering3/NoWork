import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";

// save the credentials for calendar
    export async function saveCredentials( email: string, password: string ): Promise<void> {
        await invoke('save_credentials', { email, password });
        const store = await load(".settings.json");
        await store.set("email", { value: email });
        await store.save();
    }
    // load the credentials for calendar
    export async function loadCredentials( email: string ): Promise<string> {
        const pass: string = await invoke("load_credentials", { email });
        return pass;
    }