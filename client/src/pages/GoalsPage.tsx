import React, {useContext} from "react";
import {UserContext} from "../contexts/UserContext";
import {Redirect} from "react-router-dom";
import {GoalsList} from "../components/goals/GoalsList";


export default function GoalsPage() {
    const {isLoggedIn} = useContext(UserContext);

    return <>
        {!isLoggedIn &&
        <Redirect to="/"/>
        }
        <h1>Your Goals</h1>
        <div className="box">
            <GoalsList/>
        </div>
    </>;
}
