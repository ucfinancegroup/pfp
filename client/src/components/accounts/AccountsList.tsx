//import styles from "./AccountsList.module.scss"
//import classNames from "classnames";

//const cx = classNames.bind(styles);

import PlaidLink from "../../components/accounts/PlaidLink";
import {PlaidService} from "../../services/PlaidService";
import React, {useEffect, useState} from "react";
import {PlaidApi, Account, GetAccountsAllOrUnhiddenEnum} from "../../api";
import {getRecurringFrequencyName} from "../recurring/RecurringHelpers";
import styles from "../recurring/RecurringList.module.scss";
import {formatPrice} from "../../Helpers";

type AccountsListProps = {

};


const plaidApi = new PlaidApi();

export function AccountsList(props: AccountsListProps) {

    const [plaidToken, setPlaidToken] = useState<string>();
    const [accounts, setAccounts] = useState<Account[]>();

    useEffect(() => {
        load();
    }, []);

    async function load() {
        await Promise.all([loadPlaid(), loadAccounts()]);
    }

    async function loadPlaid() {
        const token = await plaidApi.plaidLink();
        setPlaidToken(token.link_token);
    }

    async function loadAccounts() {
        setAccounts(null);
        const accounts = await plaidApi.getAccounts({
            allOrUnhidden: GetAccountsAllOrUnhiddenEnum.Unhidden,
        });

        setAccounts(accounts.accounts);
    }

    async function deleteAccount(account: Account) {
        setAccounts([...accounts.filter(a => a !== account)]);

        await plaidApi.setAccountAsHidden({
            setAccountAsHiddenPayload: {
                item_id: account.item_id,
                account_id: account.account_id,
                hide_or_not: true,
            },
        });
    }

    return <>
        {
            !accounts && <p>Loading...</p>
        }
        {
            accounts && accounts.length === 0 &&
            <div>
                <h3>👋 Welcome to Finch!</h3>
                <p>Get started by linking a financial institution to your account.
                  We use your connected accounts to track your finances and to create predictions and correlations. Try connecting a:</p>
                <ul>
                    <li>
                      🏦 Bank Account
                    </li>
                    <li>
                      📈 Investment Account
                    </li>
                    <li>
                      🧓 Retirement Account
                    </li>
                </ul>
              <p>Simply click the button below to sign in to your financial institution.</p>
            </div>
        }
        {
            accounts && accounts.length > 0 &&
            <table className="table">
              <thead>
              <tr>
                <th scope="col">Name</th>
                <th scope="col">Balance</th>
                <th scope="col">Actions</th>
              </tr>
              </thead>
              <tbody>
              {
                  accounts.map(a => <tr key={a.item_id + "/" + a.account_id}>
                      <td>{a.name}</td>
                      <td>{formatPrice(a.balance)}</td>
                      <td className={styles.actions}>
                          <i className="fa fa-times" aria-hidden="true" onClick={() => deleteAccount(a)}/>
                      </td>
                  </tr>)
              }
              </tbody>
            </table>
        }
        {
            plaidToken && <div className="mt-4">
              <PlaidLink token={plaidToken} onSuccess={() => loadAccounts()}/>
            </div>
        }
        {
            !plaidToken && accounts && <p>Loading...</p>
        }
    </>
}
