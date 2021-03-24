import React, { useEffect, useState } from "react";
import { Ranking, LeaderboardApi, BoardType } from "../../api";
import handleFetchError from "../../hooks/handleFetchError";
import styles from "./LeaderboardList.module.scss";
import classNames from "classnames";

const cx = classNames.bind(styles);

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
            for (let boardType in BoardType) {
                let ranking = await leaderboardApi.getLeaderboard({ type: BoardType[boardType] });
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
            {rankings.map(r => {
                return <div className={styles.board}>
                    <div className={styles.board__header}>
                        <div className={styles.board__name}>
                            {r.leaderboard_type}: Top <strong>{(100 - r.percentile).toFixed(1)}%</strong>
                        </div>
                    </div>
                    <div className={styles.board__progress}>
                        <div className={styles.board_bar} style={{ width: r.percentile + "%" }}>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
    return <>
        {!rankings && <p>Loading...</p>}
        {rankings && renderList(rankings)}
    </>
}