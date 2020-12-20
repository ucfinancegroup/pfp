import React, {useContext, useEffect, useState} from "react";
import {UserContext} from "../contexts/UserContext";
import PlaidLink from "../components/accounts/PlaidLink";
import {PlaidService} from "../services/PlaidService";

export default function HomePage() {
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
        <h1>Homepage</h1>
        <p>Is logged in: <strong>{isLoggedIn ? "Yes" : "No"}</strong></p>
        {
            isLoggedIn && plaidToken && <PlaidLink token={plaidToken}/>
        }
    </>;
}
