import type { TaskModification } from "../taskParser";
import { tagState } from "../taskStore.svelte";


export function parseTag(text: string): TaskModification[] {
    // we want to sort the tags from longest to shortest
    // this prevents shortest first errors from occurring
    // e.g. 'home' is prioritized over 'homework'
    let validTags = [...tagState.tags].sort((a, b) => b.name.length - a.name.length);

    let parsedTags = [];

    const regex = /#/g;
    const matches = text.matchAll(regex);

    if (matches) {
        for (const match of matches) {
            let tagName = text.slice(match.index + 1);
        
            let matchedTag = validTags.find(tag => tagName.startsWith(tag.name));
            console.log(tagName);
            
            if (matchedTag) {
                const start = match.index!;
                const end = start + matchedTag.name.length + 1;

                parsedTags.push({
                    text: match.toString(),
                    start: start,
                    end: end,
                    modType: "tag",
                    value: matchedTag
                } as TaskModification);
            }
        }
    }

    return parsedTags;
}