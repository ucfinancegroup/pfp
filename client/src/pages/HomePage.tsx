import React, {useContext, useEffect, useState} from "react";
import { Redirect } from "react-router-dom";
import {UserContext} from "../contexts/UserContext";

/**
 * The logged out user default page.
 * @constructor
 */
export default function HomePage() {
    const {isLoggedIn} = useContext(UserContext);

    return <>
        {isLoggedIn &&
            <Redirect to="/dashboard"/>
        }
        <h1>Welcome to Finch</h1>
        <p>Create an account to take control of your finances</p>
    </>;
}
