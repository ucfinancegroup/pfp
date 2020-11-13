import React from 'react';
import './App.css';
import Header from "./layout/Header";
import Routes from "./Routes";
import Container from "react-bootstrap/Container";

function App() {
    return (
        <div className="app">
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
