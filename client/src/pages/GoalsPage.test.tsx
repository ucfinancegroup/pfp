import React from 'react';
import { render, act } from '@testing-library/react';
import { BrowserRouter, Route } from 'react-router-dom';
import GoalsPage from "./GoalsPage";
import UserContextProvider from "../contexts/UserContext";
const flushPromises = () => new Promise(setImmediate);

test("Should redirect to homepage if logged out", async () => {
    const {
        container,
    } = render(
        <BrowserRouter>
            <UserContextProvider loggedIn={false}>
                <GoalsPage />
                <Route path="/">Home page</Route>
            </UserContextProvider>
        </BrowserRouter>
    );

    expect(container).toHaveTextContent(/Home page/);
});

