import type { TaskModification } from "../taskParser";
import { tagState } from "../taskStore.svelte";


export function parseTag(text: string): TaskModification[] {
    let validTags = tagState.tags;

    let parsedTags = [];

    const regex = /#\w+/g;
    const matches = text.matchAll(regex);

    if (matches) {
        for (const match of matches) {
            let tagName = match[0].slice(1);
        
            let matchedTag = validTags.find(tag => tag.name == tagName);

            if (matchedTag) {
                const start = match.index!;
                const end = start + match[0].length;

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

    console.log(parsedTags);

    return parsedTags;
}