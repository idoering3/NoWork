export interface CalendarEvent {
    summary: string;
    start: string;
    end: string;
    uid: string;
    description: string | null;
    location: string | null;
}