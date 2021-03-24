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
                            <h3>{rank.leaderboard_type}</h3>
                            {/* <p>You are in the top</p> */}
                        </div>
                        <svg className={styles.meter}>
                            <circle r="4em" cx="50%" cy="50%" stroke="green" opacity="20%"
                            stroke-width="1em"
                            fill="none">
                            </circle>
                            <circle r="4em" cx="50%" cy="50%" stroke="green"
                            stroke-width="1em"
                            stroke-dasharray={`${rank.percentile*8*Math.PI}em, 2000`}
                            fill="none">
                            </circle>
                        </svg>
                    </li>
                })}
            </ul>
        </>
    }
    return <>
        {!rankings && <p>Loading...</p>}
        {rankings && renderList(rankings)}
    </>
}