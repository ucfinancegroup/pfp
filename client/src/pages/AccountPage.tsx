import React, {useContext, useEffect, useState} from "react";
import {UserContext} from "../contexts/UserContext";
import PlaidLink from "../components/accounts/PlaidLink";
import {PlaidService} from "../services/PlaidService";
import {RecurringList} from "../components/recurring/RecurringList";
import {Redirect} from "react-router-dom";
import {PlaidApi, Account} from "../api";

const plaidApi = new PlaidApi();

export default function AccountPage() {
    const {isLoggedIn} = useContext(UserContext);
    const [plaidToken, setPlaidToken] = useState<string>();
    const [tab, setTab] = useState<"recurrings" | "accounts">("recurrings");
    const [accounts, setAccounts] = useState<Account[]>();

    useEffect(() => {
        load();
    }, [])

    async function load() {
        const token = await plaidApi.plaidLink();
        setPlaidToken(token.link_token);

        const accounts = await plaidApi.getAccounts();
        setAccounts(accounts.accounts);
        console.log(accounts);
    }

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
                {
                    !plaidToken && <p>Loading...</p>
                }
                {
                    plaidToken && <div className="mt-4">
                        <PlaidLink token={plaidToken}/>
                    </div>
                }
                </>
            }
        </div>
    </>;
}
