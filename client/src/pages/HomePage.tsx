import React, {useContext} from "react";
import {UserContext} from "../contexts/UserContext";

export default function HomePage() {
    const {isLoggedIn} = useContext(UserContext);
    return <>
        <h1>Homepage</h1>
        <p>Is logged in: <strong>{isLoggedIn ? "Yes" : "No"}</strong></p>
    </>;
}
