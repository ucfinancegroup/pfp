import styles from "./GoalsList.module.scss";
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {Goal, GoalAndStatus, GoalApi, GoalMetric, GoalNewPayload, Recurring} from "../../api";
import handleFetchError from "../../hooks/handleFetchError";
import {GoalsDialog} from "./GoalsDialog";
import {RecurringType} from "../recurring/RecurringType";

const cx = classNames.bind(styles);

type GoalsListProps = {

};

const goalApi = new GoalApi();

export function GoalsList(props: GoalsListProps) {
    let [goals, setGoals] = useState<GoalAndStatus[]>();
    const [error, setError] = useState<string>();
    const [dialogOpen, setDialogOpen] = useState<boolean>(false);
    const [dialogEditing, setDialogEditing] = useState<Goal>(null);

    useEffect(() => {
        getGoals();
    }, []);

    async function getGoals() {
        try {
            const goals = await goalApi.getGoals();
            setGoals(goals);
        } catch (e) {
            setError(await handleFetchError(e));
        }
    }

    async function dialogClosed(goal: GoalNewPayload) {
        if (goal) {
            if (dialogEditing) {
                await goalApi.updateGoal({
                    goalNewPayload: goal,
                    id: dialogEditing._id.$oid,
                });
                Object.assign(dialogEditing, goal);
                setGoals([...goals]);
            } else {
                const result = await goalApi.newGoal({
                    goalNewPayload: goal,
                });
                setGoals([...goals, result]);
            }
        }
        setDialogEditing(null);
        setDialogOpen(false);
    }

    async function deleteGoal(goal: GoalAndStatus) {
        await goalApi.deleteGoal({
            id: goal.goal._id.$oid,
        });

        setGoals([...goals.filter(g => g !== goal)]);
    }

    async function editGoal(goal: GoalAndStatus) {
        setDialogEditing(goal.goal);
        setDialogOpen(true);
    }

    function createGoal() {
        setDialogOpen(true);
    }

    return <>
        <GoalsDialog show={dialogOpen} onClose={g => dialogClosed(g)} editing={dialogEditing}/>
        {
            error && <div className="alert alert-danger" role="alert">
                {error}
            </div>
        }
        {
            !goals && !error && <p>Loading...</p>
        }
        {
            goals && goals.length === 0 && <p>You have not created any goals yet. What are you waiting for?</p>
        }
        {
            goals && goals.length > 0 && goals.map(g => {
                return <div className={styles.goal} key={g.goal._id.$oid}>
                    <div className={styles.goal__header}>
                        <div className={styles.goal__name}>
                            {g.goal.name}: <strong>{g.progress.toFixed(0)}%</strong>
                        </div>
                        <div className={styles.goal__actions}>
                            <i className="fa fa-pencil" aria-hidden="true" onClick={() => editGoal(g)}/>
                            <i className="fa fa-times" aria-hidden="true" onClick={() => deleteGoal(g)}/>
                        </div>
                    </div>
                    <div className={styles.goal__progress}>
                        <div className={styles.goal__bar} style={{width: g.progress + "%"}}>
                        </div>
                    </div>
                </div>
            })
        }
        {
            !error &&
            <button type="button" className="btn btn-primary mb-2" onClick={() => createGoal()}>
              <i className="fa fa-plus"/>
              New Goal
            </button>
        }
    </>
}
