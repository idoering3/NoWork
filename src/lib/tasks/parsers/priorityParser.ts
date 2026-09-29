import type { TaskPriority } from "../task";
import type { TaskModification } from "../taskParser";

interface PriorityMatch {
    text: string;
    priority: TaskPriority;
}

const priorityMatches: PriorityMatch[] = [
    {
        text: "!high",
        priority: "high"
    },
    {
        text: "!medium",
        priority: "medium"
    },
    {
        text: "!low",
        priority: "low"
    }
]

export function parsePriority(text: string): TaskModification | null {
    const textMatches = priorityMatches.map(match => match.text);

    const regex = new RegExp(`(${textMatches.join("|")})`);
    let result = regex.exec(text);

    if (result) {
        let start = result.index;
        let end = start + result[0].length;
        
        let match = priorityMatches.find(match => match.text === result[0]);
        // make our TaskModification now;
        let taskModification: TaskModification = {
            text: match!.text,
            start: start,
            end: end,
            modType: "priority",
            value: match!.priority
        }

        return taskModification;
    }

    return null;
}