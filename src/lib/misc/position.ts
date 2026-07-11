import { invoke } from '@tauri-apps/api/core';

export async function getGeoPosition(): Promise<{ latitude: number; longitude: number; city: string; region: string; country: string }> {
    const pos = await invoke<{ lat: number; lon: number, city:string; region: string; country:string }>('get_ip_geoposition');
    return { latitude: pos.lat, longitude: pos.lon, city: pos.city, region: pos.region, country: pos.country };
}