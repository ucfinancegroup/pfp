import React, {useContext, useEffect, useState} from "react";
import {UserContext} from "../contexts/UserContext";
import PlaidLink from "../components/accounts/PlaidLink";
import {PlaidService} from "../services/PlaidService";
import {RecurringList} from "../components/recurring/RecurringList";
import {Redirect} from "react-router-dom";
import {PlanChart} from "../components/chart/PlanChart";

/**
 * The logged in user default page
 */
export default function DashboardPage() {
    const {isLoggedIn} = useContext(UserContext);

    const [plaidToken, setPlaidToken] = useState<string>();

    useEffect(() => {
        load();
    }, [])

    async function load() {
        const token = await PlaidService.getToken();
        setPlaidToken(token.link_token);
    }

    return <>
        {!isLoggedIn &&
            <Redirect to="/"/>
        }
        <h1>Dashboard</h1>
        <div className="box">
            <PlanChart/>
        </div>
    </>;
}
