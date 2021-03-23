import React, { useEffect, useState } from "react";
import {LeaderboardList} from "../components/leaderboards/LeaderboardList";

export default function LeaderboardPage() {
    return <>
        <div className="box">
            {LeaderboardList()}
        </div>
    </>
}
