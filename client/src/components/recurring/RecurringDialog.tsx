import styles from "./RecurringDialog.module.scss"
import classNames from "classnames";
import Modal from "react-bootstrap/cjs/Modal";
import React, {useEffect, useState} from "react";
import {Recurring, RecurringApi, RecurringNewPayload, TimeIntervalTypEnum} from "../../api";
import Button from "react-bootstrap/cjs/Button";
import {RecurringType} from "./RecurringType";
import * as Yup from "yup";
import {ErrorMessage, Field, Form, Formik} from "formik";
import Dropdown from "react-bootstrap/cjs/Dropdown";
import {getRecurringType, msToDateString, recurringFrequencies} from "./RecurringHelpers";
import {addDaysToToday, dateAsInputString} from "../../Helpers";

const cx = classNames.bind(styles);

type RecurringDialogProps = {
    show: boolean;
    editing: Recurring;
    mode: RecurringType;
    onClose: (recurring: RecurringNewPayload) => void;
};

const recurringApi = new RecurringApi();

const RecurringSchema = Yup.object().shape({
    name: Yup.string().required("A name is required"),
    start: Yup.string().required(),
    end: Yup.string().required(),
    principal: Yup.number(),
    interest: Yup.number(),
    amount: Yup.number(),
    frequency: Yup.object().shape({
        typ: Yup.string().required(),
        content: Yup.number().required(),
    }).required(),
});

const initialForm = {
    name: "",
    start: dateAsInputString(new Date()),
    end: dateAsInputString(addDaysToToday(30)),
    principal: 0,
    interest: 0,
    amount: 0,
    frequency: {
        typ: TimeIntervalTypEnum.Monthly,
        content: 1,
    }
};

export function RecurringDialog(props: RecurringDialogProps) {
    const [error, setError] = useState<string>();
    const [examples, setExamples] = useState<RecurringNewPayload[]>();
    const [initialValues, setInitialValues] = useState<RecurringNewPayload>(props.editing ?? initialForm as any);
    const [isCompounding, setIsCompounding] = useState<boolean>(false);

    useEffect(() => {
        getExamples();
    }, []);

    useEffect(() => {
        if (props.editing) {
            const copy = Object.assign({}, props.editing);
            copy.amount = Math.abs(copy.amount);
            copy.start = msToDateString(copy.start) as any;

            copy.end = msToDateString(copy.end) as any;
            setIsCompounding(copy.principal !== 0 || copy.interest !== 0);
            setInitialValues(copy)
        }
    }, [props.editing]);

    async function getExamples() {
        const examples = await recurringApi.getRecurringExamples();
        for (let example of examples) {
            example.start = initialForm.start as any;
            example.end = initialForm.end as any;
        }
        setExamples(examples);
    }

    function doExample(example: RecurringNewPayload) {
        example.amount = Math.abs(example.amount);
        setInitialValues(example);
    }

    async function submit(values: RecurringNewPayload) {
        setError(null);
        if (props.mode === RecurringType.Expense) {
            values.amount *= -1;
            values.principal *= -1;
        }

        // Reset properties that shouldn't be present depending if the fixed or compounding tab was selected.
        if (isCompounding) {
            values.amount = 0;
        } else {
            values.principal = 0;
            values.interest = 0;
        }

        values.start = new Date(values.start).getTime();
        values.end = new Date(values.end).getTime();

        props.onClose(values);
        reset();
    }

    function reset() {
        setInitialValues(initialForm as any);
        setIsCompounding(false);
    }

    function close() {
        reset();
        props.onClose(null);
    }

    const currentExamples = examples && examples.filter(e => getRecurringType(e) == props.mode);

    function renderForm() {
        return <Formik key={initialValues.name}
            initialValues={initialValues}
            validationSchema={RecurringSchema}
            onSubmit={values => {
                submit({...values});
            }}
        >
            {({errors, touched, values}) => (
                <Form>

                    <ul className="nav nav-tabs mb-2 mt-4">
                        <li className="nav-item">
                            <a className={cx("nav-link", {active: !isCompounding})} onClick={() => setIsCompounding(false)}>Fixed</a>
                        </li>
                        <li className="nav-item">
                            <a className={cx("nav-link", {active: isCompounding})} onClick={() => setIsCompounding(true)}>Compounding</a>
                        </li>
                    </ul>


                    <div className="form-row">
                        <div className="col">
                            <div className="form-group">
                                <label>Name:</label>
                                <Field name="name" type="text"
                                       className={cx("form-control", {"is-invalid": errors.name && touched.name})}/>
                                <div className="invalid-feedback"><ErrorMessage name="name"/></div>
                            </div>
                        </div>
                    </div>

                    {isCompounding &&
                        <div className="form-row">
                            <div className="col">
                              <div className="form-group">
                                <label>Principal $:</label>
                                <Field name="principal" type="number"
                                       className={cx("form-control", {"is-invalid": errors.principal && touched.principal})}/>
                                <small className="form-text text-muted">The starting amount</small>
                                <div className="invalid-feedback"><ErrorMessage name="principal"/></div>
                              </div>
                            </div>
                            <div className="col">
                              <div className="form-group">
                                <label>Interest %:</label>
                                <Field name="interest" type="number"
                                       className={cx("form-control", {"is-invalid": errors.interest && touched.interest})}/>
                                <small className="form-text text-muted">The annual compound interest</small>
                                <div className="invalid-feedback"><ErrorMessage name="interest"/></div>
                              </div>
                            </div>
                        </div>
                    }

                    <div className="form-row">
                        {!isCompounding &&
                        <div className="col">
                          <div className="form-group">
                            <label>$ {props.mode === RecurringType.Expense ? "Cost" : "Amount"}:</label>
                            <Field name="amount" type="number"
                                   className={cx("form-control", {"is-invalid": errors.amount && touched.amount})}/>
                            <div className="invalid-feedback"><ErrorMessage name="amount"/></div>
                          </div>
                        </div>
                        }
                        <div className="col">
                            <div className="form-group">
                                <label>times:</label>
                                <Field name="frequency.content" type="number"
                                       className={cx("form-control", {"is-invalid": errors.frequency?.content && touched.frequency?.content})}/>
                                <div className="invalid-feedback"><ErrorMessage name="frequency.content"/></div>
                            </div>
                        </div>
                        <div className="col">
                            <div className="form-group">
                                <label>per:</label>
                                <Field as="select" name="frequency.typ"
                                       className={cx("form-control", {"is-invalid": errors.frequency?.typ && touched.frequency?.typ})}>
                                    {
                                        recurringFrequencies.map(c => <option value={c.type} key={c.type}>{c.name}</option>)
                                    }
                                </Field>
                                <div className="invalid-feedback"><ErrorMessage name="frequency.typ"/></div>
                            </div>
                        </div>
                    </div>


                    <div className="form-row">
                        <div className="col">
                            <div className="form-group">
                                <label>Start:</label>
                                <Field name="start" type="date"
                                       className={cx("form-control", {"is-invalid": errors.start && touched.start})}/>
                                <div className="invalid-feedback"><ErrorMessage name="start"/></div>
                            </div>
                        </div>
                        <div className="col">
                            <div className="form-group">
                                <label>End:</label>
                                <Field name="end" type="date"  validate={() => {
                                    if (values.end <= values.start) return "End must be after start"; }}
                                       className={cx("form-control", {"is-invalid": errors.end && touched.end})}/>
                                <div className="invalid-feedback"><ErrorMessage name="end"/></div>
                            </div>
                        </div>
                    </div>

                    <button className="btn btn-primary mt-4" type="submit" disabled={Object.keys(touched).length === 0 || Object.keys(errors).length !== 0}>
                        {props.editing ? "Save" : "Add"}
                    </button>
                </Form>
            )}
        </Formik>
    }

    return <Modal show={props.show} onHide={() => close()}>
        <Modal.Header closeButton>
            <Modal.Title>{props.editing ? "Edit" : "Add"} {props.mode === RecurringType.Income ? "Income" : "Expense"}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <>
                {
                    error && <div className="alert alert-danger" role="alert">
                        {error}
                    </div>
                }
                {
                    !props.editing && examples && currentExamples.length > 0 && <>
                        <p>Choose from an example, or input your own.</p>
                        <strong>Examples: </strong>
                        {
                            currentExamples.map(e => <Button key={e.name}
                                variant="primary" className={styles.example}
                                onClick={() => doExample(e)}>{e.name}</Button>)
                        }
                    </>
                }
                {
                    renderForm()
                }
            </>
        </Modal.Body>
    </Modal>
}
