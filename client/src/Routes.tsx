import {
    BrowserRouter as Router,
    Switch,
    Route,
} from "react-router-dom";
import React from "react";
import HomePage from "./pages/HomePage";
import RegisterPage from "./pages/RegisterPage";
import LoginPage from "./pages/LoginPage";

export default function Routes() {
    return <Router>
        <Switch>
            <Route exact path="/">
                <HomePage/>
            </Route>
            <Route path="/login">
                <LoginPage/>
            </Route>
            <Route path="/register">
                <RegisterPage/>
            </Route>
        </Switch>
    </Router>
}
