
// type for our date formats
export type DateFormatName = keyof typeof dateFormats;


// print the date in various formats for the homepage
export const dateFormats = {
    slashStandard: getSlashStandardDate,
    slashEuropean: getSlashEuropeanDate,
    dashIso: getDashIsoDate,
    textStandard: getTextStandardDate,
    textEuropean: getTextEuropeanDate,
    monthDay: getMonthDayDate,
    dayOfWeekAndMonth: getDayOfWeekAndTextStandardDateShort,
    dayOfWeekFull: getDayOfWeekAndFullDate,
    dayOfWeekShort: getDayOfWeekShort,
    dayOfWeekOnly: getDayOfWeek
};

export const dateFormatOptions = Object.entries(dateFormats).map(
    ([key, formatter]) => ({
        value: key as DateFormatName,
        label: formatter(new Date())
    })
);

// format: MM/DD/YYYY
export function getSlashStandardDate(date: Date): string {
    return date.toLocaleDateString('en-US');
}

export function getSlashEuropeanDate(date: Date): string {
    return date.toLocaleDateString('en-GB');
}

export function getDashIsoDate(date: Date): string {
    return date.toISOString().slice(0, 10);
}

export function getTextStandardDate(date: Date): string {
    return date.toLocaleDateString('en-US', {
        month: 'long',
        day: 'numeric',
        year: 'numeric'
    });
}

export function getTextEuropeanDate(date: Date): string {
    return date.toLocaleDateString('en-GB', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
    });
}

export function getMonthDayDate(date: Date): string {
    return date.toLocaleDateString('en-US', {
        month: 'long',
        day: 'numeric'
    });
}

export function getDayOfWeekAndTextStandardDateShort(date: Date): string {
    return date.toLocaleDateString('en-US', {
        weekday: 'long',
        month: 'short',
        day: 'numeric'
    });
}

export function getDayOfWeekAndFullDate(date: Date): string {
    return date.toLocaleDateString('en-US', {
        weekday: 'long',
        month: 'long',
        day: 'numeric',
        year: 'numeric'
    });
}

export function getDayOfWeekShort(date: Date): string {
    return date.toLocaleDateString('en-US', {
        weekday: 'short'
    });
}

export function getDayOfWeek(date: Date): string {
    return date.toLocaleDateString('en-US', {
        weekday: 'long'
    });
}

