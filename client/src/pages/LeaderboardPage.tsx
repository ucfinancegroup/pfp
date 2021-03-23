import React, { useEffect, useState } from "react";
import { Ranking, LeaderboardApi, BoardType } from "../api";
import handleFetchError from "../hooks/handleFetchError";

const leaderboardApi = new LeaderboardApi();

export default function LeaderboardPage() {
    const [rankings, setRankings] = useState<Ranking[]>();
    const [error, setError] = useState<string>();

    useEffect(() => {
        getRankings();
    }, []);

    async function getRankings() {
        try {
            let results = [];
            for (let type in BoardType) {
                let ranking = await leaderboardApi.getLeaderboard({type: BoardType.Savings});
                results.push(ranking);
            }
            setRankings(results);

        } catch (e) {
            setError(await handleFetchError(e));
            console.log(e);
        }
        console.log(rankings);
    }

    return <>
        <div>
            <p>{JSON.stringify(rankings)}</p>
        </div>
    </>
}
