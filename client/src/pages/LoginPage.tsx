import React, {useContext, useState} from "react";
import * as Yup from 'yup';
import {ErrorMessage, Field, Form, Formik} from "formik";
import classNames from "classnames/bind";
import {UserContext} from "../contexts/UserContext";
import { useHistory } from "react-router-dom";
import {UserApi} from "../api";
import handleFetchError from "../hooks/handleFetchError";

const cx = classNames;

const LoginSchema = Yup.object().shape({
    password: Yup.string().required('Password is required.'),
    email: Yup.string().required('Email is required.'),
});

const userApi = new UserApi();

export default function LoginPage() {
    const [error, setError] = useState<string>();
    const [loading, setLoading] = useState<boolean>();
    const {setIsLoggedIn} = useContext(UserContext);
    const router = useHistory();

    async function submit({email, password}: { email: string, password: string }) {
        // Prevent submitting the form twice.
        if (loading) return;
        setLoading(true);

        try {
            await userApi.loginUser({
                loginPayload: {
                    email,
                    password,
                }
            });

            setIsLoggedIn(true);
            router.push("/dashboard");
        } catch (e) {
            setError(await handleFetchError(e));
        } finally {
            setLoading(false);
        }
    }

    return <>
        <h1>Log In</h1>
        {
            error && <div className="alert alert-danger" role="alert">
                {error}
            </div>
        }
        <div className="row">
            <div className="col-md-4">
                <Formik
                    initialValues={{
                        password: '',
                        email: '',
                    }}
                    validationSchema={LoginSchema}
                    onSubmit={values => {
                        submit({...values});
                    }}
                >
                    {({errors, touched}) => (
                        <Form>
                            <div className="form-group">
                                <label>E-Mail:</label>
                                <Field name="email" type="email"
                                       className={cx("form-control", {"is-invalid": errors.email && touched.email})}/>
                                <div className="invalid-feedback"><ErrorMessage name="email"/></div>
                            </div>

                            <div className="form-group">
                                <label>Password:</label>
                                <Field name="password" type="password"
                                       className={cx("form-control", {"is-invalid": errors.password && touched.password})}/>
                                <div className="invalid-feedback"><ErrorMessage name="password"/></div>
                            </div>
                            <button className="btn btn-primary" type="submit"
                                    disabled={!!errors.password || !!errors.email}>
                                <span>{loading ? "Logging in..." : "Log In"}</span>
                            </button>
                        </Form>
                    )}
                </Formik>
            </div>
        </div>
    </>
}
