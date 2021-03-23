import {render} from "@testing-library/react";
import React from "react";
import {GoalsList} from "./GoalsList";
import {GoalAndStatus, GoalMetric} from "../../api";

test("Should render list of goals", async () => {
    const goals: GoalAndStatus[] = [
        {
            progress: 0.5,
            goal: {
                _id: {
                    $oid: "dfsfsdf"
                },
                name: "Test Goal",
                start: new Date().getTime(),
                end: new Date().getTime(),
                threshold: 100,
                metric: GoalMetric.Savings,
            }
        }
    ];

    const {
        container,
    } = render(
        <GoalsList goals={goals}/>
    );

    expect(container).toHaveTextContent(/Test Goal/);
    expect(container).toHaveTextContent(/50%/);
});

