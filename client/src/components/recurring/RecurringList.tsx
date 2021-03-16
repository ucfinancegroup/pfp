import styles from "./RecurringList.module.scss";
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {Recurring, RecurringApi, RecurringNewPayload} from "../../api";
import handleFetchError from "../../hooks/handleFetchError";
import {RecurringDialog} from "./RecurringDialog";
import {RecurringType} from "./RecurringType";
import {epochToDate, getRecurringFrequencyName, getRecurringType} from "./RecurringHelpers";
import {dateAsInputString} from "../../Helpers";

const cx = classNames.bind(styles);

type RecurringListProps = {

};

const recurringApi = new RecurringApi();

export function RecurringList(props: RecurringListProps) {
    const [recurrings, setRecurrings] = useState<Recurring[]>();
    const [error, setError] = useState<string>();
    const [dialogOpen, setDialogOpen] = useState<boolean>(false);
    const [dialogEditing, setDialogEditing] = useState<Recurring>(null);
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

    async function dialogClosed(recurring: RecurringNewPayload) {
        if (recurring) {
            if (dialogEditing) {
                await recurringApi.updateRecurring({
                    recurringNewPayload: recurring,
                    id: dialogEditing._id.$oid,
                });
                Object.assign(dialogEditing, recurring);
                setRecurrings([...recurrings]);
            } else {
                const result = await recurringApi.newRecurring({
                    recurringNewPayload: recurring
                });
                setRecurrings([...recurrings, result]);
            }
        }

        setDialogEditing(null);
        setDialogOpen(false);
    }

    async function deleteRecurring(recurring: Recurring) {
        await recurringApi.deleteRecurring({
            id: recurring._id.$oid,
        });

        setRecurrings([...recurrings.filter(r => r !== recurring)]);
    }

    async function editRecurring(recurring: Recurring) {
        setDialogMode(recurring.amount < 0 ? RecurringType.Expense : RecurringType.Income);
        setDialogEditing(recurring);
        setDialogOpen(true);
    }

    if (!recurrings) return <>Loading...</>;

    const incomes = recurrings && recurrings.filter(r => getRecurringType(r) === RecurringType.Income);
    const expenses = recurrings && recurrings.filter(r => getRecurringType(r) === RecurringType.Expense);

    function renderTable(recurrings: Recurring[]) {
        if (recurrings.length === 0) return <p>None yet</p>
        return <table className="table">
            <thead>
            <tr>
                <th scope="col">Name</th>
                <th scope="col">Amount</th>
                <th scope="col">Frequency</th>
                <th scope="col">From</th>
                <th scope="col">Until</th>
                <th scope="col">Actions</th>
            </tr>
            </thead>
            <tbody>
            {
                recurrings.map(r => <tr key={r._id.$oid}>
                    <td>{r.name}</td>
                    <td>
                        {
                            r.interest === 0&& Math.abs(r.amount)
                        }
                        {
                            r.interest !== 0 && <>{r.principal} @ {Math.abs(r.interest)}%</>
                        }
                    </td>
                    <td>{getRecurringFrequencyName(r.frequency.content, r.frequency.typ)}</td>
                    <td>{dateAsInputString(epochToDate(r.start))}</td>
                    <td>{dateAsInputString(epochToDate(r.end))}</td>
                    <td className={styles.actions}>
                        <i className="fa fa-times" aria-hidden="true" onClick={() => deleteRecurring(r)}/>
                        <i className="fa fa-pencil" aria-hidden="true" onClick={() => editRecurring(r)}/>
                    </td>
                </tr>)
            }
            </tbody>
        </table>
    }

    return <>
        <RecurringDialog show={dialogOpen} mode={dialogMode} onClose={r => dialogClosed(r)} onDelete={r => deleteRecurring(r)} editing={dialogEditing}/>
        {
            error && <div className="alert alert-danger" role="alert">
                {error}
            </div>
        }
        {
            !recurrings && !error && <p>Loading...</p>
        }
        <div>
            {
                <>
                    <div className="d-flex justify-content-between">
                        <h3>Expenses</h3>
                        <button type="button" className="btn btn-primary mb-2" onClick={() => addExpense()}>
                            <i className="fa fa-plus"/>
                            Add Expense</button>
                    </div>

                    {renderTable(expenses)}

                    <div className="d-flex justify-content-between mt-4">
                      <h3>Income</h3>
                      <button type="button" className="btn btn-primary mb-2" onClick={() => addIncome()}>
                          <i className="fa fa-plus"/>
                          Add Income</button>
                    </div>

                    {renderTable(incomes)}
                </>
            }
        </div>
    </>
}
