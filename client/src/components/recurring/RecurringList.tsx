import styles from "./RecurringList.module.scss";
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {Recurring, RecurringApi} from "../../api";
import handleFetchError from "../../hooks/handleFetchError";
import {RecurringDialog} from "./RecurringDialog";
import {RecurringType} from "./RecurringType";
import {getRecurringType} from "./RecurringHelpers";

const cx = classNames.bind(styles);

type RecurringListProps = {

};

const recurringApi = new RecurringApi();

export function RecurringList(props: RecurringListProps) {
    const [recurrings, setRecurrings] = useState<Recurring[]>();
    const [error, setError] = useState<string>();
    const [dialogOpen, setDialogOpen] = useState<boolean>(false);
    const [dialogMode, setDialogMode] = useState<RecurringType>();

    useEffect(() => {
        getRecurrings();
    }, []);

    async function getRecurrings() {
        try {
            const recurrings = await recurringApi.getRecurrings();
            setRecurrings(recurrings);
        } catch (e) {
            setError(await handleFetchError(e));
        }
    }

    function addIncome() {
        setDialogMode(RecurringType.Income);
        setDialogOpen(true);
    }

    function addExpense() {
        setDialogMode(RecurringType.Expense);
        setDialogOpen(true);
    }

    function dialogClosed() {
        setDialogOpen(false);
    }

    async function deleteRecurring(recurring: Recurring) {
        await recurringApi.deleteRecurring({
            id: recurring._id.$oid,
        });

        setRecurrings([...recurrings.filter(r => r !== recurring)]);
    }

    if (!recurrings) return <>Loading...</>;

    const incomes = recurrings && recurrings.filter(r => getRecurringType(r) === RecurringType.Income);
    const expenses = recurrings && recurrings.filter(r => getRecurringType(r) === RecurringType.Expense);

    function renderTable(recurrings: Recurring[]) {
        if (recurrings.length === 0) return <span>None</span>
        return <table className="table">
            <thead>
            <tr>
                <th scope="col">Name</th>
                <th scope="col">Amount</th>
                <th scope="col">Frequency</th>
                <th scope="col">Actions</th>
            </tr>
            </thead>
            <tbody>
            {
                recurrings.map(r => <tr>
                    <td>{r.name}</td>
                    <td>${r.amount}</td>
                    <td>{r.frequency.typ}</td>
                    <td>
                        <button onClick={() => deleteRecurring(r)}>
                            Remove
                        </button>
                    </td>
                </tr>)
            }
            </tbody>
        </table>
    }

    return <>
        <RecurringDialog show={dialogOpen} mode={dialogMode} onClose={() => dialogClosed()}/>
        {
            error && <div className="alert alert-danger" role="alert">
                {error}
            </div>
        }
        <div className="alert alert-primary" role="alert">
            Finch predicts your finances using your connected bank account, as well as income and expenses that you enter.
        </div>
        <div className="d-grid gap-4 d-md-block mb-4">
            <button type="button" className="btn btn-success" onClick={() => addIncome()}>Add Income</button>
            {' '}
            <button type="button" className="btn btn-danger" onClick={() => addExpense()}>Add Expense</button>
        </div>
        <div>
            {
                recurrings.length === 0 &&
                <p>You have no incomes or expenses yet.</p>
            }
            {
                recurrings.length > 0 &&
                <>
                  <h3>Expenses</h3>
                    {renderTable(expenses)}
                  <h3 className="mt-4">Income</h3>
                    {renderTable(incomes)}
                </>
            }
        </div>
    </>
}
