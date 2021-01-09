import styles from "./RecurringDialog.module.scss"
import classNames from "classnames";
import Modal from "react-bootstrap/cjs/Modal";
import React, {useEffect, useState} from "react";
import {NewRecurringRequest, Recurring, RecurringApi, RecurringNewPayload, TimeIntervalTypEnum} from "../../api";
import Button from "react-bootstrap/cjs/Button";
import {RecurringType} from "./RecurringType";
import * as Yup from "yup";
import {ErrorMessage, Field, Form, Formik} from "formik";
import Dropdown from "react-bootstrap/cjs/Dropdown";
import {getRecurringType} from "./RecurringHelpers";

const cx = classNames.bind(styles);

type RecurringDialogProps = {
    show: boolean;
    mode: RecurringType;
    onClose: () => void;
};

const recurringApi = new RecurringApi();

const RecurringSchema = Yup.object().shape({
    name: Yup.string().required("A name is required"),
    start: Yup.date(),
    end: Yup.date(),
    principal: Yup.number(),
    interest: Yup.number(),
    amount: Yup.number(),
    frequency: Yup.object().shape({
        typ: Yup.string(),
        content: Yup.number()
    })
});

const initialForm = {
    name: "",
    start: (new Date()).getTime(),
    end:  (new Date()).getTime(),
    principal: 0,
    interest: 0,
    amount: 0,
    frequency: {
        typ: TimeIntervalTypEnum.Monthly,
        content: 0,
    }
};

const frequencies = [
    {
        name: "Annually",
        type: "annually",
        content: 1,
    },
    {
        name: "Monthly",
        type: "monthly",
        content: 2,
    },
    {
        name: "Weekly",
        type: "weekly",
        content: 3,
    },
    {
        name: "Daily",
        type: "daily",
        content: 4,
    }
]

export function RecurringDialog(props: RecurringDialogProps) {
    const [error, setError] = useState<string>();
    const [examples, setExamples] = useState<RecurringNewPayload[]>();
    const [initialValues, setInitialValues] = useState<RecurringNewPayload>(initialForm);
    const [enablePricipal, setEnablePricipal] = useState<boolean>(false);
    const [enableInterest, setEnableInterest] = useState<boolean>(false);

    useEffect(() => {
        getExamples();
    }, []);

    async function getExamples() {
        const examples = await recurringApi.getRecurringExamples();
        setExamples(examples);
    }

    function doExample(example: RecurringNewPayload) {
        setInitialValues(example);
    }

    async function submit(values: RecurringNewPayload) {
        const result = await recurringApi.newRecurring({
            recurringNewPayload: values,
        });
        console.log(result);
    }

    function close() {
        setInitialValues(initialForm);
        props.onClose();
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
            {({errors, touched}) => (
                <Form>
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

                    <div className="form-row">
                        <div className="col">
                            <div className="form-group">
                                <label>$ Amount:</label>
                                <Field name="amount" type="input"
                                       className={cx("form-control", {"is-invalid": errors.amount && touched.amount})}/>
                                <div className="invalid-feedback"><ErrorMessage name="amount"/></div>
                            </div>
                        </div>
                        <div className="col">
                            <div className="form-group">
                                <label>Frequency:</label>
                                <Field as="select" name="country"
                                       className={cx("form-control", {"input--error": errors.frequency && touched.frequency})}>
                                    {
                                        frequencies.map(c => <option value={c.content as any} key={c.type}>{c.name}</option>)
                                    }
                                </Field>
                                <div className="invalid-feedback"><ErrorMessage name="frequency"/></div>
                            </div>
                        </div>
                    </div>

                    <div className="form-row">
                        {enablePricipal &&
                            <div className="col">
                                <div className="form-group">
                                    <label>Principal $:</label>
                                    <Field name="principal" type="input"
                                           className={cx("form-control", {"is-invalid": errors.principal && touched.principal})}/>
                                    <small className="form-text text-muted">The starting amount</small>
                                    <div className="invalid-feedback"><ErrorMessage name="principal"/></div>
                                </div>
                            </div>
                        }
                        {enableInterest &&
                            <div className="col">
                                <div className="form-group">
                                    <label>Interest %:</label>
                                    <Field name="interest" type="input"
                                           className={cx("form-control", {"is-invalid": errors.interest && touched.interest})}/>
                                    <small className="form-text text-muted">The annual compound interest</small>
                                    <div className="invalid-feedback"><ErrorMessage name="interest"/></div>
                                </div>
                            </div>
                        }
                    </div>
                    {(!enablePricipal || !enableInterest) &&
                        <Dropdown>
                            <Dropdown.Toggle id="dropdown-basic">
                                Advanced
                            </Dropdown.Toggle>

                            <Dropdown.Menu>
                                {!enablePricipal &&
                                    <Dropdown.Item onClick={() => setEnablePricipal(true)}>Add Principal</Dropdown.Item>
                                }
                                {!enableInterest &&
                                    <Dropdown.Item onClick={() => setEnableInterest(true)}>Add Interest</Dropdown.Item>
                                }
                            </Dropdown.Menu>
                        </Dropdown>
                    }

                    <button className="btn btn-primary mt-4" type="submit">
                        Add
                    </button>
                </Form>
            )}
        </Formik>
    }

    return <Modal show={props.show} onHide={() => close()}>
        <Modal.Header closeButton>
            <Modal.Title>Add {props.mode === RecurringType.Income ? "Income" : "Expense"}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <>
                {
                    error && <div className="alert alert-danger" role="alert">
                        {error}
                    </div>
                }
                {
                    examples && currentExamples.length > 0 && <>
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
