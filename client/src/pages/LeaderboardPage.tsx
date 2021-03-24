import React, {useContext, useEffect, useState} from "react";
import { LeaderboardList } from "../components/leaderboards/LeaderboardList";
import {UserContext} from "../contexts/UserContext";
import {Redirect, useHistory, useLocation} from "react-router-dom";

export default function LeaderboardPage() {
    const {isLoggedIn} = useContext(UserContext);
    return <>
        {!isLoggedIn &&
            <Redirect to="/"/>
        }
        <h1>Your Leaderboards</h1>
        <div className="box">
            <LeaderboardList />
        </div>
    </>
}
