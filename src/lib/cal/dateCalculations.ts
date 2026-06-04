export function stringToDate(raw: string): Date {
    // All-day event compact: 20260521
    if (/^\d{8}$/.test(raw)) {
        return new Date(`${raw.slice(0,4)}-${raw.slice(4,6)}-${raw.slice(6,8)}`);
    }

    // All-day event hyphenated: 2026-05-30
    if (/^\d{4}-\d{2}-\d{2}$/.test(raw)) {
        const [y, m, d] = raw.split('-').map(Number);
        return new Date(y, m - 1, d); // local midnight, no UTC shift
    }

    // DateTime with Z: 20260521T133500Z
    if (/^\d{8}T\d{6}Z$/.test(raw)) {
        return new Date(`${raw.slice(0,4)}-${raw.slice(4,6)}-${raw.slice(6,8)}T${raw.slice(9,11)}:${raw.slice(11,13)}:${raw.slice(13,15)}Z`);
    }

    // DateTime without Z (floating/local): 20260521T133500
    if (/^\d{8}T\d{6}$/.test(raw)) {
        return new Date(`${raw.slice(0,4)}-${raw.slice(4,6)}-${raw.slice(6,8)}T${raw.slice(9,11)}:${raw.slice(11,13)}:${raw.slice(13,15)}`);
    }

    // ISO 8601 with offset: 2026-05-21T09:00:00+02:00
    if (/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}([+-]\d{2}:\d{2}|Z)$/.test(raw)) {
        return new Date(raw);
    }

    return new Date(raw);
}