import {Recurring, RecurringNewPayload} from "../../api";
import {RecurringType} from "./RecurringType";

export function getRecurringType(recurring: RecurringNewPayload | Recurring): RecurringType {
    if (recurring.amount < 0)
        return RecurringType.Expense;
    else
        return RecurringType.Income;
}
