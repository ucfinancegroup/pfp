import {
    BrowserRouter as Router,
    Switch,
    Route,
} from "react-router-dom";
import React from "react";
import HomePage from "./pages/HomePage";
import RegisterPage from "./pages/RegisterPage";
import LoginPage from "./pages/LoginPage";
import DashboardPage from "./pages/DashboardPage";
import AccountPage from "./pages/AccountPage";
import GoalsPage from "./pages/GoalsPage";

export default function Routes() {
    return <Router>
        <Switch>
            <Route exact path="/">
                <HomePage/>
            </Route>
            <Route exact path="/dashboard">
                <DashboardPage/>
            </Route>
            <Route exact path="/goals">
                <GoalsPage/>
            </Route>
            <Route exact path="/account">
                <AccountPage/>
            </Route>
            <Route exact path="/login">
                <LoginPage/>
            </Route>
            <Route exact path="/register">
                <RegisterPage/>
            </Route>
        </Switch>
    </Router>
}
