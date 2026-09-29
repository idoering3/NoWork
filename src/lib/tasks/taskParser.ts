import { parseDate } from "./parsers/dateParser";
import { parsePriority } from "./parsers/priorityParser";
import { parseTag } from "./parsers/tagParser";
import type { CreateTask, Tag, TaskPriority } from "./task";

// we want to make a struct that captures info about the text + modification
type ModificationType =
    | "dueDate"
    | "priority"
    | "tag";

export interface TaskModification {
    text: string;
    // where the text is located in the original string
    start: number;
    end: number;
    modType: ModificationType;
    // to avoid later re-parse
    value: unknown;
}

export function parseTaskText(text: string): TaskModification[] {
    let modifications: TaskModification[] = [];
    text = text.toLowerCase();

    const date = parseDate(text);
    if (date !== null)
        modifications.push(date);

    const priority = parsePriority(text);
    if (priority !== null)
        modifications.push(priority);

    const tags = parseTag(text);
    if (tags.length !== 0)
        modifications.push(...tags);

    return modifications;
}

export function proposeModifications(taskName: string, modifications: TaskModification[]): CreateTask {
    modifications.sort((a, b) => a.start - b.end);
    // we have to strip from the text any modification overlap.
    let name = stripTaskName(taskName, modifications);

    // need to first get all the modifications into a new create task
    let dueDateMod = modifications.find(mod => mod.modType === "dueDate");
    let dueDate = dueDateMod?.value as Date | null ?? null;

    let priorityMod = modifications.find(mod => mod.modType === "priority");
    let priority = priorityMod?.value as TaskPriority ?? null as TaskPriority;

    let tagMod = modifications.filter(mod => mod.modType === "tag");
    let tags: Tag[] = tagMod.map(tag => tag.value as Tag);

    let task = {
        name: name,
        dueDate: dueDate,
        priority: priority,
        tags: tags
    }

    return task;
};

function stripTaskName(taskName: string, modifications: TaskModification[]): string {
    let result = "";
    let cursor = 0;
    for (const mod of modifications) {
        result += taskName.slice(cursor, mod.start);
        cursor = mod.end;
    }
    result += taskName.slice(cursor);

    return result;
}