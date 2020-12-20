import React, {useContext, useState} from "react";
import * as Yup from 'yup';
import {ErrorMessage, Field, Form, Formik} from "formik";
import classNames from "classnames/bind";
import {UserContext} from "../contexts/UserContext";
import {useHistory} from "react-router-dom";

const cx = classNames;

const RegisterSchema = Yup.object().shape({
    password: Yup.string()
        .min(8, 'Password must be at least 8 characters.')
        .max(100, 'Password is too long.')
        .required('Password is required.'),
    email: Yup.string()
        .email('Invalid email.')
        .required('Email is required.'),
    firstName: Yup.string().required('Last name is required'),
    lastName: Yup.string().required('Last name is required'),
});

export default function RegisterPage() {
    const [error, setError] = useState<string>();
    const [loading, setLoading] = useState<boolean>();
    const {setIsLoggedIn} = useContext(UserContext);
    const router = useHistory();

    async function submit({email, password, firstName, lastName}:
                              { email: string, password: string, firstName: string, lastName: string }) {
        // Prevent submitting the form twice.
        if (loading) return;

        try {
            setLoading(true);

            const res = await fetch("/api/signup", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    email,
                    password,
                    first_name: firstName,
                    last_name: lastName,
                    income: 10, // TODO REMOVE THIS
                }),
            });
            const text = await res.text();
            const data = JSON.parse(text);

            if (res.ok) {
                setIsLoggedIn(true);
                router.push("/");
            } else {
                setError(data.message || "An error occurred");
            }
        } catch (e) {
            setError(e.message);
        } finally {
            setLoading(false);
        }
    }

    return <>
        <h1>Create an Account</h1>
        {
            error && <div className="alert alert-danger" role="alert">
                {error}
            </div>
        }
        <div className="row">
            <div className="col-md-6">
                <Formik
                    initialValues={{
                        password: '',
                        email: '',
                        firstName: '',
                        lastName: '',
                    }}
                    validationSchema={RegisterSchema}
                    onSubmit={values => {
                        submit({...values});
                    }}
                >
                    {({errors, touched}) => (
                        <Form>
                            <div className="form-row">
                                <div className="col">
                                    <div className="form-group">
                                        <label>First Name:</label>
                                        <Field name="firstName" type="text"
                                               className={cx("form-control", {"is-invalid": errors.firstName && touched.firstName})}/>
                                        <div className="invalid-feedback"><ErrorMessage name="firstName"/></div>
                                    </div>
                                </div>
                                <div className="col">
                                    <div className="form-group">
                                        <label>Last Name:</label>
                                        <Field name="lastName" type="text"
                                               className={cx("form-control", {"is-invalid": errors.lastName && touched.lastName})}/>
                                        <div className="invalid-feedback"><ErrorMessage name="lastName"/></div>
                                    </div>
                                </div>
                            </div>
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
                                <span>{loading ? "Loading..." : "Create Account"}</span>
                            </button>
                        </Form>
                    )}
                </Formik>
            </div>
        </div>
    </>
}
