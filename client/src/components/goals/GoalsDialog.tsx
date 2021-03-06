import styles from "./GoalsDialog.module.scss";
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {
    Goal,
    GoalApi,
    GoalMetric,
    GoalNewPayload,
} from "../../api";
import * as Yup from "yup";
import Modal from "react-bootstrap/cjs/Modal";
import Button from "react-bootstrap/cjs/Button";
import {ErrorMessage, Field, Form, Formik} from "formik";
import {epochToDateString} from "../recurring/RecurringHelpers";
import {goalMetrics} from "./GoalHelpers";
import {addDays, dateAsInputString} from "../../Helpers";

const cx = classNames.bind(styles);

type GoalsDialogProps = {
    show: boolean;
    editing: Goal;
    onClose: (goal: GoalNewPayload) => void;
};

const goalApi = new GoalApi();

const GoalSchema = Yup.object().shape({
    name: Yup.string().required("A name is required"),
    start: Yup.string().required(),
    end: Yup.string().required(),
    threshold: Yup.number().required("A threshold is required"),
    metric: Yup.string().required(),
});

const initialForm = {
    name: "",
    start: dateAsInputString(new Date()),
    end: dateAsInputString(addDays(new Date(), 30)),
    threshold: 1000,
    metric: GoalMetric.Income,
};

export function GoalsDialog(props: GoalsDialogProps) {
    const [error, setError] = useState<string>();
    const [examples, setExamples] = useState<GoalNewPayload[]>();
    const [initialValues, setInitialValues] = useState<GoalNewPayload>(props.editing ?? initialForm as any);

    useEffect(() => {
        getExamples();
    }, []);

    useEffect(() => {
        if (props.editing) {
            const copy = Object.assign({}, props.editing);
            copy.start = epochToDateString(copy.start) as any;
            copy.end = epochToDateString(copy.end) as any;
            setInitialValues(copy)
        }
    }, [props.editing]);

    async function getExamples() {
        const examples = await goalApi.getGoalExamples();
        for (let example of examples) {
            example.start = initialForm.start as any;
            example.end = initialForm.end as any;
        }
        setExamples(examples);
    }

    function doExample(example: GoalNewPayload) {
        const clone = Object.assign({}, example);
        clone.threshold = Math.abs(example.threshold);
        setInitialValues(clone);
    }

    async function submit(values: GoalNewPayload) {
        setError(null);
        if (typeof values.start === 'string')
            values.start = new Date(values.start).getTime() / 1000;
        if (typeof values.end === 'string')
            values.end = new Date(values.end).getTime() / 1000;

        props.onClose(values);
        reset();
    }

    function reset() {
        setInitialValues(initialForm as any);
    }

    function close() {
        reset();
        props.onClose(null);
    }


    const currentExamples = examples; // Add filtering here if needed.

    function renderForm() {
        return <Formik key={initialValues.name}
                       initialValues={initialValues}
                       validationSchema={GoalSchema}
                       onSubmit={values => {
                           submit({...values});
                       }}
        >
            {({errors, touched, values, isValid}) => (
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
                                <label>$ Threshold:</label>
                                <Field name="threshold" type="number"
                                       className={cx("form-control", {"is-invalid": errors.threshold && touched.threshold})}/>
                                <div className="invalid-feedback"><ErrorMessage name="threshold"/></div>
                            </div>
                        </div>
                        <div className="col">
                            <div className="form-group">
                                <label>Metric:</label>
                                <Field as="select" name="metric"
                                       className={cx("form-control", {"is-invalid": errors.metric && touched.metric})}>
                                    {
                                        goalMetrics.map(c => <option value={c.value} key={c.value}>{c.name}</option>)
                                    }
                                </Field>
                                <div className="invalid-feedback"><ErrorMessage name="metric"/></div>
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

                    <button className="btn btn-primary" type="submit" disabled={!isValid}>
                        {props.editing ? "Save" : "Add"}
                    </button>
                </Form>
            )}
        </Formik>
    }

    return <Modal show={props.show} onHide={() => close()}>
        <Modal.Header closeButton>
            <Modal.Title>{props.editing ? "Edit" : "Add"} Goal</Modal.Title>
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
