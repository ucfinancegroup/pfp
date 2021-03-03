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
        const accounts = await plaidApi.getAccounts({
            allOrUnhidden: GetAccountsAllOrUnhiddenEnum.All,
        });

        setAccounts(accounts.accounts);
    }

    async function deleteAccount(account: Account) {
        await plaidApi.deleteAccount({
            id: account.item_id,
        });

        setAccounts([...accounts.filter(a => a !== account)]);
    }

    return <>
        {
            !plaidToken && <p>Loading...</p>
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
                  accounts.map(a => <tr key={a.item_id}>
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
    </>
}
