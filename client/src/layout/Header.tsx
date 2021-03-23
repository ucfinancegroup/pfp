import Navbar from "react-bootstrap/Navbar";
import Nav from "react-bootstrap/Nav";
import React, {useContext} from "react";
import {UserContext} from "../contexts/UserContext";
import Cookies from "js-cookie";
import Container from "react-bootstrap/Container";
import styles from "./Header.module.scss";
import classNames from "classnames";
import {UserApi} from "../api";
const cx = classNames.bind(styles);

const userApi = new UserApi();

export default function Header() {
    const {isLoggedIn, setIsLoggedIn} = useContext(UserContext);

    async function logout() {
        setIsLoggedIn(false);
        await userApi.logoutUser();
        Cookies.remove("finch-sid");
    }

    return <header>
        <Navbar expand="lg" bg="dark" variant="dark">
            <Container>
                <Navbar.Brand href="/">
                    <img id="logo"
                        alt=""
                        src="/logo.png"
                        className={cx(styles.logo, "d-inline-block", "align-top")}
                    />
                    Finch
                </Navbar.Brand>
                {isLoggedIn &&
                <Navbar.Collapse>
                      <Nav.Link href="/dashboard">Dashboard</Nav.Link>
                      <Nav.Link href="/goals">Goals</Nav.Link>
                      <Nav.Link href="/leaderboard">Leaderboards</Nav.Link>
                </Navbar.Collapse>
                }
                <Navbar.Collapse className="justify-content-end">
                    {!isLoggedIn && <>
                      <Nav.Link href="/login">Log In</Nav.Link>
                      <Nav.Link className="btn btn-primary" href="/register">Get Started</Nav.Link>
                    </>
                    }
                    {isLoggedIn && <>
                      <Nav.Link href="/account">Account</Nav.Link>
                      <Nav.Link onClick={() => logout()}>Logout</Nav.Link>
                    </>
                    }
                </Navbar.Collapse>
            </Container>
        </Navbar>
    </header>
}
