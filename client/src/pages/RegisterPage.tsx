import React, {useContext, useEffect, useState} from "react";
import * as Yup from 'yup';
import {ErrorMessage, Field, Form, Formik} from "formik";
import classNames from "classnames/bind";
import {UserContext} from "../contexts/UserContext";
import {useHistory} from "react-router-dom";
import {Location, UserApi} from "../api";
import handleFetchError from "../hooks/handleFetchError";

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

const userApi = new UserApi();

export default function RegisterPage() {
    const [error, setError] = useState<string>();
    const [loading, setLoading] = useState<boolean>();
    const {setIsLoggedIn} = useContext(UserContext);
    const router = useHistory();
    const [location, setLocation] = useState<Location>({
        has_location: false,
        lat: 0,
        lon: 0,
    });

    useEffect(() => {
        getLocation();
    });

    function getLocation() {
         navigator.geolocation.getCurrentPosition(pos => {
             const newLocation = {
                 has_location: true,
                 lat: pos.coords.latitude,
                 lon: pos.coords.longitude,
             };
             console.log("Got location", newLocation);
             setLocation(newLocation);
         }, e => console.warn("Unable to get location", e));
    }

    async function submit({email, password, firstName, lastName}:
                              { email: string, password: string, firstName: string, lastName: string }) {
        // Prevent submitting the form twice.
        if (loading) return;
        setLoading(true);

        try {
            await userApi.signupUser({
                signupPayload: {
                    email,
                    password,
                    first_name: firstName,
                    last_name: lastName,
                    income: 100,
                    location,
                }
            })
            setIsLoggedIn(true);
            router.push("/dashboard");
        } catch (e) {
            setError(await handleFetchError(e));
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
