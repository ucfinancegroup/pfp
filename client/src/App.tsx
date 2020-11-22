import React from 'react';
import Header from "./layout/Header";
import Routes from "./Routes";
import Container from "react-bootstrap/Container";
import UserContextProvider from "./contexts/UserContext";

function App() {
    return (
        <UserContextProvider>
            <Header/>
            <div className="content">
                <Container>
                    <Routes/>
                </Container>
            </div>
        </UserContextProvider>
    );
}

export default App;
