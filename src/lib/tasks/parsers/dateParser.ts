import type { TaskModification } from "../taskParser";

// checks for parsing of dates...
export function parseDate(text: string): TaskModification | null {
    const matches: TaskModification[] = [];
    const textMatches = dateMatches.map(match => match.text);
    // need to convert to a regex flag
    const regex = new RegExp(`\\b(${textMatches.join("|")})\\b`, "i");

    const wordMatch = regex.exec(text);
    
    if (wordMatch) {
        const match = dateMatches.find(
            match => match.text === wordMatch[0]
        );

        if (match) {
            matches.push({
                text: match.text,
                start: wordMatch.index,
                end: wordMatch.index + wordMatch[0].length,
                modType: "dueDate",
                value: match.date
            });
        }
    }

    // For the format DD/MM/YYYY
    const customRegex = /\b(\d{2})\/(\d{2})\/(\d{4})\b/;
    const customMatch = customRegex.exec(text);

    if (customMatch) {
        const day = Number(customMatch[1]);
        const month = Number(customMatch[2]);
        const year = Number(customMatch[3]);

        matches.push({
            text: customMatch[0],
            start: customMatch.index,
            end: customMatch.index + customMatch[0].length,
            modType: "dueDate",
            value: new Date(year, month - 1, day)
        });
    }

    // Nothing matched
    if (matches.length === 0) {
        return null;
    }

    // Return the leftmost match
    return matches.reduce((leftmost, current) =>
        current.start < leftmost.start ? current : leftmost
    );
}

interface DateMatch {
    text: string;
    date: Date;
}




// today, tomorrow, etc
const today = new Date();
const tomorrow = new Date();
tomorrow.setDate(tomorrow.getDate() + 1);

// days of the week
const weekdayMap: Record<string, number> = {
    sunday: 0,
    monday: 1,
    tuesday: 2,
    wednesday: 3,
    thursday: 4,
    friday: 5,
    saturday: 6,
};

function getNextDayOfWeek(targetDay: number) {
    const today = new Date();

    const daysAway = (targetDay - today.getDay() + 7) % 7;
    const result = new Date();
    result.setDate(result.getDate() + daysAway);

    return result;
}

// custom dates formatted DD/MM/YYYY
// TODO add more customization e.g. MM/DD/YYYY in settings


const dateMatches: DateMatch[] = [
    {
        text: "tomorrow",
        date: tomorrow
    },
    {
        text: "today",
        date: today
    },
    {
        text: "sunday",
        date: getNextDayOfWeek(weekdayMap.sunday)
    },
    {
        text: "monday",
        date: getNextDayOfWeek(weekdayMap.monday)
    },
    {
        text: "tuesday",
        date: getNextDayOfWeek(weekdayMap.tuesday)
    },
    {
        text: "wednesday",
        date: getNextDayOfWeek(weekdayMap.wednesday)
    },
    {
        text: "thursday",
        date: getNextDayOfWeek(weekdayMap.thursday)
    },
    {
        text: "friday",
        date: getNextDayOfWeek(weekdayMap.friday)
    },
    {
        text: "saturday",
        date: getNextDayOfWeek(weekdayMap.saturday)
    },
];