import Navbar from "react-bootstrap/Navbar";
import Nav from "react-bootstrap/Nav";
import React, {useContext} from "react";
import {UserContext} from "../contexts/UserContext";
import Cookies from "js-cookie";

export default function Header() {
    const {isLoggedIn, setIsLoggedIn} = useContext(UserContext);

    function logout() {
        Cookies.remove("finch-sid");
        setIsLoggedIn(false);
    }

    return <header>
        <Navbar bg="dark" variant="dark">
            <Navbar.Brand href="/">Finch App</Navbar.Brand>
            <Navbar.Collapse className="justify-content-end">
                {!isLoggedIn && <>
                  <Nav.Link href="/login">Log In</Nav.Link>
                  <Nav.Link className="btn btn-primary" href="/register">Get Started</Nav.Link>
                </>
                }
                {isLoggedIn && <>
                  <Nav.Link onClick={() => logout()}>Logout</Nav.Link>
                </>
                }
            </Navbar.Collapse>
        </Navbar>
    </header>
}
