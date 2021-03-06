import React, {useContext, useEffect, useState} from "react";
import {UserContext} from "../contexts/UserContext";
import {RecurringList} from "../components/recurring/RecurringList";
import {Redirect, useHistory, useLocation} from "react-router-dom";
import {AccountsList} from "../components/accounts/AccountsList";
export default function AccountPage() {
    const {isLoggedIn} = useContext(UserContext);
    const location = useLocation();
    const history = useHistory();
    const [tab, setTab] = useState<"recurrings" | "accounts">(new URLSearchParams(location.search).get("tab") as any || "recurrings");

    function gotoTab(tab: "recurrings" | "accounts") {
        setTab(tab);
        history.replace("/account?tab=" + tab);
    }

    return <>
        {!isLoggedIn &&
            <Redirect to="/"/>
        }
        <h1>Your Account</h1>
        <ul className="nav nav-pills mt-4 mb-4">
            <li className="nav-item" onClick={() => gotoTab("recurrings")}>
                <a className={"nav-link " + (tab === "recurrings" ? "active" : "")}>Income & Expenses</a>
            </li>
            <li className="nav-item" onClick={() => gotoTab("accounts")}>
                <a className={"nav-link " + (tab === "accounts" ? "active" : "")}>Connected Accounts</a>
            </li>
        </ul>
        <div className="box">
            {
                tab === "recurrings" &&
                <RecurringList/>
            }
            {
                tab === "accounts" && <>
                    <AccountsList/>
                </>
            }
        </div>
    </>;
}
