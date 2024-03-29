import styles from "./AllocationEditorDialog.module.scss"
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {Allocation, AllocationProportion, AssetClassAndApy, AssetClassesApi, AssetClassTypEnum} from "../../api";
import Modal from "react-bootstrap/cjs/Modal";
import {dateAsInputString} from "../../Helpers";
import {epochToDate} from "../recurring/RecurringHelpers";
import {Form} from "formik";

const cx = classNames.bind(styles);

type AllocationEditorDialogProps = {
    show: boolean,
    allocations: Allocation[],
    editing: Allocation,
    creating: Date,
    onClose: (allocations: Allocation[]) => void;
};

const assetsApi = new AssetClassesApi();

export function AllocationEditorDialog(props: AllocationEditorDialogProps) {
    const [error, setError] = useState<string>();
    const [classes, setClasses] = useState<AssetClassAndApy[]>();
    const [name, setName] = useState<string>("");
    const [date, setDate] = useState<string>(dateAsInputString(new Date()));
    const [assets, setAssets] = useState<AllocationProportion[]>([]);
    useEffect(() => {
        if (props.show)
            load();
    }, [props.show]);

    async function load() {
        const assets = await assetsApi.getAssetClasses();
        assets.push({
            apy: 1,
            _class: {
                typ: AssetClassTypEnum.Custom,
                content: "Custom",
            }
        });
        setClasses(assets);

        if (!props.editing) {
            let basedOn: Allocation;
            for (let allocation of props.allocations) {
                if (epochToDate(allocation.date) < props.creating)
                    basedOn = allocation;
                else
                    break;
            }
            setDate(dateAsInputString(props.creating));
            setAssets(basedOn.schema.map(x => Object.assign({_react: Math.random()}, x)));
        } else {
            setDate(dateAsInputString(epochToDate(props.editing.date)));
            setAssets(props.editing.schema.map(x => Object.assign({_react: Math.random()}, x)));
        }
    }

    function close() {
        props.onClose(null);
    }

    function getReturn(apy) {
        return Math.round((apy - 1) * 100 * 1000) / 1000;

    }

    function addAsset() {
        const assetClass = classes[0];
        const newAsset: AllocationProportion = {
            asset: {
                name: "",
                _class: assetClass._class,
                annualized_performance: assetClass.apy,
            },
            proportion: 0,
        };
        (newAsset as any)._react = Math.random();
        setAssets([...assets, newAsset]);
    }

    function removeAsset(asset: AllocationProportion) {
        const index = assets.indexOf(asset);
        assets.splice(index, 1);
        setAssets([...assets]);
    }

    function getClass(value: string) {
        return classes.find(c => c._class.typ == value);
    }


    const allocationTotal = assets.length === 0 ? 0 : assets.map(a => a.proportion).reduce((a, b) => a + b);
    const allocationRemaining = 100 - allocationTotal;

    function save() {
        if (!props.editing) {
            const newAllocation: Allocation = {
                _id: null,
                description: name,
                date: new Date(date).getTime() / 1000,
                schema: assets,
            }
            const newAllocations = [...props.allocations, newAllocation].sort((a, b) =>
                a.date - b.date);
            props.onClose(newAllocations);
        } else {
            props.editing.schema = assets;
            const newAllocations = [...props.allocations];
            props.onClose(newAllocations);
        }
    }

    function deleteThis() {
        props.onClose([...props.allocations.filter(a => a._id.$oid !== props.editing._id.$oid)]);
    }

    return <Modal show={props.show} onHide={() => close()}  dialogClassName="modal-large">
        <Modal.Header closeButton>
            <Modal.Title>Edit Asset Allocation</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <div className="form-row">
                <div className="col">
                    <div className="form-group">
                        <label>Name:</label>
                        <input className="form-control" type="text" name="name" value={name}
                               placeholder="Describe this allocation change"
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
            <h5>Assets:</h5>
            <div className="d-flex flex-row align-items-center mb-2">
                <button className="btn btn-primary" onClick={() => addAsset()}>
                    <i className="fa fa-plus"/>
                    Add Asset
                </button>
                <span className={cx("ml-2", {
                    "text-danger": allocationRemaining > 0,
                })}>
                    {
                        allocationRemaining > 0 &&
                   <> <strong>Unallocated: </strong>{allocationRemaining.toFixed(2)}%</>
                }
                </span>
            </div>
            <table className="table">
                <thead>
                <tr>
                    <th>
                        Name
                    </th>
                    <th>
                        Allocation
                    </th>
                    <th>
                        Asset Class (% return)
                    </th>
                    <th>
                        Actions
                    </th>
                </tr>
                </thead>
                <tbody>
                {
                    assets.map((a, i) =>
                        <tr key={(a as any)._react} className={styles.class}>
                            <td className={styles.class__name}>
                                <input className="form-control" type="text" value={a.asset.name} onChange={e => {
                                    a.asset.name = e.target.value;
                                    setAssets([...assets]);
                                }}/>
                            </td>
                            <td>
                                <span className={styles.slider__container}>
                                    <input type="range" min="0" max="100"
                                           className={cx(styles.slider, "form-control")}
                                        value={a.proportion}  onChange={e => {
                                        const maxValue = Math.min(100, a.proportion + allocationRemaining);
                                        let value = parseFloat(e.target.value);
                                        value = Math.min(value, maxValue);
                                        a.proportion = value;
                                        setAssets([...assets]);
                                    }}/>
                                    <span>{a.proportion.toFixed(2)}%</span>
                                </span>
                            </td>
                            <td className={styles.nowrap}>
                                <select value={a.asset._class.typ} className="form-control d-inline-block"  onChange={e => {
                                    const class1 = getClass(e.target.value);
                                    a.asset._class = class1._class;
                                    a.asset.annualized_performance = class1.apy;
                                    setAssets([...assets]);
                                }}>
                                    {classes.map(c => <option key={c._class.typ} value={c._class.typ}>{c._class.typ}</option>)}
                                </select>
                                {
                                    a.asset._class.typ === AssetClassTypEnum.Custom &&
                                      <span>
                                        <input type="number" min="0" max="100" className={cx("form-control", styles.apy__input)}
                                               value={getReturn(a.asset.annualized_performance)} onChange={e => {
                                            a.asset.annualized_performance = 1 + (parseFloat(e.target.value) / 100);
                                            setAssets([...assets]);
                                        }}/>
                                        <span>%</span>
                                      </span>
                                }
                                {
                                    a.asset._class.typ !== AssetClassTypEnum.Custom &&
                                    <span className={styles.apy}>({getReturn(a.asset.annualized_performance).toFixed(1)}%)</span>
                                }
                            </td>
                            <td className={styles.actions}>
                                <i className="fa fa-times" aria-hidden="true" onClick={() => removeAsset(a)}/>
                            </td>
                        </tr>)
                }
                </tbody>
            </table>
            <button className="btn btn-primary" onClick={() => save()} disabled={allocationRemaining > 0}>
                <i className="fa fa-save"/>
                Save Assets
            </button>
            {
                props.editing && <button className="btn btn-outline-danger float-right" onClick={() => deleteThis()}>
                  <i className="fa fa-times"/>
                  Delete
                </button>
            }
        </Modal.Body>
    </Modal>
}
