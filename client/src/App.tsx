import React from 'react';
import Header from "./layout/Header";
import Routes from "./Routes";
import Container from "react-bootstrap/Container";

function App() {
    return (
        <div>
            <Header/>
            <div className="content">
                <Container>
                    <Routes/>
                </Container>
            </div>
        </div>
    );
}

export default App;
