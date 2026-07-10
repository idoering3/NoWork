import { invoke } from '@tauri-apps/api/core';

export async function getGeoPosition(): Promise<{ latitude: number; longitude: number }> {
    const pos = await invoke<{ lat: number; lon: number }>('get_ip_geoposition');
    return { latitude: pos.lat, longitude: pos.lon };
}