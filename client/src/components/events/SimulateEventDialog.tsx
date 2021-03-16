import styles from "./SimulateEventDialog.module.scss"
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {
    AssetClassAndApy,
    AssetClassChange,
    AssetClassesApi,
    EventApi,
    Event,
} from "../../api";
import Modal from "react-bootstrap/cjs/Modal";
import {dateAsInputString} from "../../Helpers";
import Button from "react-bootstrap/cjs/Button";
import {epochToDate} from "../recurring/RecurringHelpers";

const cx = classNames.bind(styles);

const assetsApi = new AssetClassesApi();
const eventsApi = new EventApi();

type SimulateEventDialogProps = {
    show: boolean,
    events: Event[],
    editing: Event,
    creating: Date,
    onClose: (events: Event[]) => void;
};

export function SimulateEventDialog(props: SimulateEventDialogProps) {
    const [error, setError] = useState<string>();
    const [name, setName] = useState<string>("");
    const [date, setDate] = useState<string>(dateAsInputString(new Date()));
    const [assetClasses, setAssetClasses] = useState<AssetClassAndApy[]>([]);
    const [selectedAssetClass, setSelectedAssetClass] = useState<AssetClassAndApy>();
    const [changes, setChanges] = useState<AssetClassChange[]>([]);
    const [examples, setExamples] = useState<Event[]>([]);

    useEffect(() => {
        if (props.show)
            load();
    }, [props.show]);

    useEffect(() => {
        loadExamples();
    }, []);

    async function loadExamples() {
        setExamples( await eventsApi.getEventExamples());
    }

    async function load() {
        const classes = await assetsApi.getAssetClasses();
        setChanges([]);
        setExamples(examples);
        setSelectedAssetClass(classes[0]);
        setAssetClasses(classes);
        if (!props.editing) {
            setDate(dateAsInputString(props.creating));
        } else {
            setName(props.editing.name);
            setDate(dateAsInputString(epochToDate(props.editing.start)));
            setChanges(props.editing.transforms.map(x => Object.assign({_react: Math.random()}, x)));
        }
    }

    function close() {
        props.onClose(null);
    }

    function save() {
        if (!props.editing) {
            const newEvent: Event = {
                _id: null,
                name: name,
                start: new Date(date).getTime() / 1000,
                transforms: changes,
            }
            const newEvents = [...props.events, newEvent].sort((a, b) =>
                a.start - b.start);
            props.onClose(newEvents);
        } else {
            props.editing.transforms = changes;
            const newEvents = [...props.events];
            props.onClose(newEvents);
        }
    }

    function getClass(value: string) {
        return assetClasses.find(c => c._class.typ == value);
    }

    function addChange(cls: AssetClassAndApy) {
        const newChange: AssetClassChange = {
            _class: cls._class,
            change: 0,
        };

        (newChange as any)._react = Math.random();

        setSelectedAssetClass(null);
        setChanges([...changes, newChange]);
    }

    function removeClass(cls: AssetClassChange) {
        const index = changes.indexOf(cls);

        changes.splice(index, 1);
        setChanges([...changes]);
    }

    function doExample(example: Event) {
        const clone = Object.assign({}, example);
        setName(clone.name);
        setChanges([...clone.transforms.map(x => Object.assign({_react: Math.random()}, x))]);
    }

    const remainingClasses = assetClasses &&
        assetClasses.filter(ac => !changes.find(change => change._class.typ === ac._class.typ));
        const selectedAssetClassActual = selectedAssetClass ?? remainingClasses[0];


    return <Modal show={props.show} onHide={() => close()} dialogClassName="modal-large">
        <Modal.Header closeButton>
            <Modal.Title>Simulate Event</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <div className="form-row">
                <div className="col">
                    <div className="form-group">
                        <label>Name:</label>
                        <input className="form-control" type="text" name="name" value={name}
                               placeholder="Describe this event"
                               onChange={e => setName(e.target.value)}/>
                    </div>
                </div>
                <div className="col">
                    <div className="form-group">
                        <label>Effective Date:</label>
                        <input className="form-control" type="date" name="date" value={date}
                               onChange={e => setDate(e.target.value)}/>
                    </div>
                </div>
            </div>
            <h5>Changes:</h5>
            <p>Select an asset class to modify the performance of it. For example, -10% performance on stocks means
                a 10% drop in the value of any of your stock assets.</p>

            <div className="mb-3">
            <strong>Examples: </strong>
            {
                examples.map(e => <Button key={e.name}
                                          variant="primary" className={styles.example}
                                          onClick={() => doExample(e)}>{e.name}</Button>)
            }
            </div>
            {
                assetClasses &&
                <>
                    {remainingClasses.length > 0 &&
                    <div className="d-flex align-items-center">
                      <select value={selectedAssetClassActual._class.typ}
                              style={{maxWidth: "200px"}}
                              className="form-control d-inline-block mr-2"
                              onChange={e => setSelectedAssetClass(getClass(e.target.value))}>
                          {
                              remainingClasses.map(c => <option key={c._class.typ}
                                                                value={c._class.typ}>{c._class.typ}</option>)
                          }
                      </select>
                      <button className="d-inline-block btn btn-primary" onClick={() => addChange(selectedAssetClassActual)}>
                        Add Change
                      </button>
                    </div>
                    }
                    <div className={styles.classes}>
                    {
                        changes.map((c, i) =>
                            <div key={(c as any)._react} className={styles.class}>
                            <strong className={styles.class__name}>{c._class.typ}</strong>
                                <span className={styles.actions}><i className="fa fa-times" aria-hidden="true" onClick={() => removeClass(c)}/></span>
                                 <span className={styles.slider__container}>
                                <input type="range" min="-100" max="100"
                                       className={cx(styles.slider, "form-control")}
                                       value={c.change}  onChange={e => {
                                    c.change = parseFloat(e.target.value);
                                    setChanges([...changes]);
                                }}/>
                                <strong className={cx({
                                    "text-danger": c.change < 0,
                                    "text-success": c.change > 0,
                                })}>{c.change >= 0 ? "+" : ""}{c.change.toFixed(2)}%</strong>
                            </span>
                            </div>)
                    }
                    </div>
                </>
            }
            <button className="btn btn-primary" onClick={() => save()}>
                <i className="fa fa-save"/>
                Save
            </button>
        </Modal.Body>
    </Modal>
}
