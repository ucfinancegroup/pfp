import React, { useEffect, useState } from "react";
import { LeaderboardList } from "../components/leaderboards/LeaderboardList";

export default function LeaderboardPage() {
    return <>
        <h1>Your Leaderboards</h1>
        <div className="box">
            <LeaderboardList />
        </div>
    </>
}
