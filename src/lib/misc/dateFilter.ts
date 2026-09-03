// enum equivalent
export const DateFilter = {
    Today: "today",
    ThisWeek: "this-week",
    NotDue: "not-due",
    None: null,
} as const;

export type DateFilter = typeof DateFilter[keyof typeof DateFilter];