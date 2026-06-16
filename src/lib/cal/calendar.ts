export interface CalendarEvent {
    summary: string;
    start: string;
    end: string;
    uid: string;
    description: string | null;
    location: string | null;
}

export function getTime(date: string): number {
    return new Date(date).getTime();
}

function overlaps(a: CalendarEvent, b: CalendarEvent): boolean {
    return (
        getTime(a.start) < getTime(b.end) &&
        getTime(a.end) > getTime(b.start)
    );
}

function contains(a: CalendarEvent, b: CalendarEvent): boolean {
    return (
        getTime(a.start) <= getTime(b.start) &&
        getTime(a.end) >= getTime(b.end) &&
        a.uid !== b.uid
    );
}

/**
 * Returns how far inward this event should shift.
 *
 * Rules:
 * - If another event fully contains this one -> shift inward
 * - If another overlapping event starts earlier -> shift inward
 * - Longer/earlier events stay anchored
 */
export function getMaxConcurrent(
    target: CalendarEvent,
    events: CalendarEvent[]
): number {
    let offset = 0;

    for (const event of events) {
        if (event.uid === target.uid) continue;

        // ignore unrelated events
        if (!overlaps(target, event)) continue;

        // nested inside another event
        if (contains(event, target)) {
            offset++;
        }
        // partial overlap: earlier event takes priority
        else if (getTime(event.start) < getTime(target.start)) {
            offset++;
        }
    }

    return offset;
}