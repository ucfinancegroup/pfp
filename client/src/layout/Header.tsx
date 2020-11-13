import Navbar from "react-bootstrap/Navbar";
import React from "react";

export default function Header() {
    return <header>
        <Navbar bg="light">
            <Navbar.Brand href="#home">Finch App</Navbar.Brand>
        </Navbar>
    </header>
}
