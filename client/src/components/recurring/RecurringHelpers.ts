import {Recurring, RecurringNewPayload} from "../../api";
import {RecurringType} from "./RecurringType";

export function getRecurringType(recurring: RecurringNewPayload | Recurring): RecurringType {
    if (recurring.amount < 0)
        return RecurringType.Expense;
    else
        return RecurringType.Income;
}

export function getRecurringFrequencyName(content: number, type: string) {
    const name = recurringFrequencies.find(f => f.type === type).name.toLocaleLowerCase();

    if (content === 1) {
        return "every " + name;
    } else {
        return "every " + content + " " + name + "s";
    }
}

export function epochToDate(seconds: number) {
    return new Date(seconds * 1000);
}

export function epochToDateString(seconds: number) {
    const date = epochToDate(seconds);
    const day = date.getUTCDate();
    const month = (date.getUTCMonth() + 1);
    return date.getUTCFullYear() + "-" + (month < 10 ? "0" + month : month) + "-" + (day < 10 ? "0" + day : day);
}

export const recurringFrequencies = [
    {
        name: "Year",
        type: "annually"
    },
    {
        name: "Month",
        type: "monthly",
    },
    {
        name: "Week",
        type: "weekly",
    },
    {
        name: "Day",
        type: "daily",
    },
]
