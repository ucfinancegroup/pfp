import React, {useContext} from "react";
import {UserContext} from "../contexts/UserContext";
import {Redirect} from "react-router-dom";

export default function LeaderboardPage() {
    const {isLoggedIn} = useContext(UserContext);

    return <>
        {!isLoggedIn &&
        <Redirect to="/"/>
        }
        <h1>Leaderboard</h1>
        <div className="box">
        </div>
    </>;
}
