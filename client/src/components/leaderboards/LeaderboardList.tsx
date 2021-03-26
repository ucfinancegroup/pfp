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
    const [tab, setTab] = useState<BoardType>();

    function gotoTab(tab: BoardType) {
        setTab(tab);
    }
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

    console.log(styles);

    function renderList(rankings: Ranking[]) {
        return <>
            <ul className="nav nav-pills mt-4 mb-4">
                {rankings.map(rank => {
                    return <li className={styles.board}>
                        <div>
                            <div className={styles.center}>
                                <h3>{rank.leaderboard_type}</h3>
                                <p>You are better than <strong>
                                    <span className=
                                        {(rank.percentile < 50) ? styles.red : styles.green}>
                                        {(100 - rank.percentile).toFixed(1)}%
                            </span></strong> of similar users by {rank.leaderboard_type}.</p>
                            </div>
                            <div className={styles.info}><i className="fa fa-info" aria-hidden="true"/></div>
                        </div>
                        <svg className={styles.meter}>
                            <defs>
                                <linearGradient id="linear" x1="0%" y1="0%" x2="100%" y2="0%">
                                    <stop offset="0%" stop-color="#1fb08e" />
                                    <stop offset="100%" stop-color="#5adec0" />
                                </linearGradient>
                            </defs>
                            <circle r="4em" cx="70%" cy="50%" stroke="green" opacity="20%"
                                stroke-width="1em"
                                fill="none">
                            </circle>
                            <circle r="4em" cx="70%" cy="50%" stroke="url(#linear)"
                                stroke-width="1em"
                                stroke-dasharray={`${rank.percentile / 100 * 8 * Math.PI}em, 2000`}
                                fill="none">
                            </circle>

                        </svg>
                    </li>
                })}
            </ul>
        </>
    }
    return <>
        {error && <p>Error: {error}</p>}
        {!error && !rankings && <p>Loading...</p>}
        {!error && rankings && renderList(rankings)}
    </>
}