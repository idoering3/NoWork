import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from '@tauri-apps/plugin-notification';
import { resolveResource } from "@tauri-apps/api/path";

export async function sendNotif() {
    try {
        // 1. Get the absolute path to the file
        // Ensure "assets/sad_chief_crates.png" is listed in "resources" in tauri.conf.json!
        const resourcePath = await resolveResource("assets/sad_chief_crates.gif");

        if (!resourcePath) {
            console.error("Could not find resource path");
            return;
        }

        const normalizedPath = resourcePath.replace(/\\/g, '/');
        const encodedPath = normalizedPath.split('/').map(encodeURIComponent).join('/');

        const attachmentUrl = `file:///${encodedPath}`;

        // 3. Permission Check
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }

        // 4. Send
        if (permissionGranted) {
            sendNotification({ 
                title: 'Chief Needs You To Finish the Fight!', 
                body: 'You have a task due today...',
                attachments: [
                    {
                        id: 'chief-image',
                        url: attachmentUrl, // Passing the file:// URL
                    },
                ], 
            });
        }
    } catch (err) {
        console.error("Notification Error:", err);
    }
}