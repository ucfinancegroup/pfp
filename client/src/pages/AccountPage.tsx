import React, {useContext, useEffect, useState} from "react";
import {UserContext} from "../contexts/UserContext";
import {RecurringList} from "../components/recurring/RecurringList";
import {Redirect} from "react-router-dom";
import {AccountsList} from "../components/accounts/AccountsList";
export default function AccountPage() {
    const {isLoggedIn} = useContext(UserContext);
    const [tab, setTab] = useState<"recurrings" | "accounts">("recurrings");

    return <>
        {!isLoggedIn &&
            <Redirect to="/"/>
        }
        <h1>Your Account</h1>
        <ul className="nav nav-pills mt-4 mb-4">
            <li className="nav-item" onClick={() => setTab("recurrings")}>
                <a className={"nav-link " + (tab === "recurrings" ? "active" : "")}>Income & Expenses</a>
            </li>
            <li className="nav-item" onClick={() => setTab("accounts")}>
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
                  <h3>Connected Accounts</h3>
                    <AccountsList/>
                </>
            }
        </div>
    </>;
}
