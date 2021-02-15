import styles from "./InsightStack.module.scss"
import classNames from "classnames";
import {Insight, InsightsApi} from "../../api";
import {useEffect, useState} from "react";
import React from "react";
import TinderCard from "react-tinder-card";

const cx = classNames.bind(styles);

type InsightStackProps = {

};

const insightsApi = new InsightsApi();

export function InsightStack(props: InsightStackProps) {
    const [insights, setInsights] = useState<Insight[]>();

    useEffect(() => {
        load();
    }, []);

    async function load() {
        setInsights(await insightsApi.getInsights());
    }

    async function dismiss(insight: Insight) {
        const index = insights.indexOf(insight);
        insights.splice(index, 1);
        setInsights([...insights]);

        await insightsApi.dismissInsight({
            id: insight._id.$oid
        });
    }

    if (!insights || insights.length === 0)
        return null;

    return <div className={styles.stack}>
        <div className={styles.badge}>{insights.length}</div>
        {
            insights.filter(i => !i.dismissed).map((i, index) =>
                    <TinderCard onCardLeftScreen={() => dismiss(i)} key={i._id.$oid}>
                        <div className={cx(styles.card, {[styles.shadow]: index === 0})}>
                            <div className={styles.image}>
                                <img src={i.imageURL}/>
                            </div>
                            <div className={styles.body}>
                            <h5 className={styles.title}>
                                {i.title}
                            </h5>
                            <div className={styles.description}>
                                {i.description}
                            </div>
                            </div>
                        </div>
                        </TinderCard>

            )
        }
    </div>
}
