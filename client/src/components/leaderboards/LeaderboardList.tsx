import React, { useEffect, useState } from "react";
import { Ranking, LeaderboardApi, BoardType } from "../../api";
import handleFetchError from "../../hooks/handleFetchError";

const leaderboardApi = new LeaderboardApi();

export function LeaderboardList() {
    const [rankings, setRankings] = useState<Ranking[]>();
    const [error, setError] = useState<string>();

    useEffect(() => {
        getRankings();
    }, []);

    async function getRankings() {
        try {
            let results = [];
            for (let type in BoardType) {
                let ranking = await leaderboardApi.getLeaderboard({ type: BoardType.Savings });
                results.push(ranking);
            }
            setRankings(results);

        } catch (e) {
            setError(await handleFetchError(e));
        }
        console.log(rankings);
    }

    function renderList(rankings: Ranking[]) {
        return <div>
            {rankings.map(r => <p>
                    {JSON.stringify(r)}
                </p>)}
        </div>
    }
    return <>
    {!rankings && <p>Loading...</p> }
    {rankings && renderList(rankings)}
    </>
}