import styles from "./AllocationEditorDialog.module.scss"
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {Allocation, AllocationChange, AssetClassAndApy, AssetClassesApi, AssetClassTypEnum} from "../../api";
import Modal from "react-bootstrap/cjs/Modal";
import {dateAsInputString} from "../../Helpers";

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
    const [assets, setAssets] = useState<AllocationChange[]>([]);
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

        // If we are creating a new allocation, get the last allocation to base this on.
        let basedOn: Allocation;
        for (let allocation of props.allocations) {
            if (new Date(allocation.date) < props.creating)
                basedOn = allocation;
            else
                break;
        }
        setDate(dateAsInputString(props.creating));
        basedOn.schema.forEach(a => (a as any)._react = Math.random());
        setAssets(basedOn.schema);

        // If we are editing an allocation...
    }

    function close() {
        props.onClose(null);
    }

    function getReturn(apy) {
        return Math.round((apy - 1) * 100 * 1000) / 1000;

    }

    function addAsset() {
        const assetClass = classes[0];
        const newAsset: AllocationChange = {
            asset: {
                name: "",
                _class: assetClass._class,
                annualized_performance: assetClass.apy,
            },
            change: 0,
        };
        (newAsset as any)._react = Math.random();
        setAssets([...assets, newAsset]);
    }

    function removeAsset(asset: AllocationChange) {
        const index = assets.indexOf(asset);
        assets.splice(index, 1);
        setAssets([...assets]);
    }

    function getClass(value: string) {
        return classes.find(c => c._class.typ == value);
    }

    function save() {
        if (!props.editing) {
            const newAllocation: Allocation = {
                description: name,
                date: new Date(date).getTime(),
                schema: assets,
            }
            const newAllocations = [...props.allocations, newAllocation].sort((a, b) =>
                a.date - b.date);
            props.onClose(newAllocations);
        } else {

        }
    }

    const allocationTotal = assets.length === 0 ? 0 : assets.map(a => a.change).reduce((a, b) => a + b);
    const allocationRemaining = 100 - allocationTotal;

    return <Modal show={props.show} onHide={() => close()}  dialogClassName="modal-large">
        <Modal.Header closeButton>
            <Modal.Title>Edit Assets</Modal.Title>
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
            <h3>Assets:</h3>
            <p><strong>Unallocated: </strong>{allocationRemaining.toFixed(2)}%</p>
            <button className="btn btn-primary" onClick={() => addAsset()}>
                Add Asset
            </button>
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
                        Asset Class
                    </th>
                    <th>
                        Actions
                    </th>
                </tr>
                </thead>
                <tbody>
                {
                    assets.map((a, i) =>
                        <tr key={(a as any)._react}>
                            <td className={styles.class__name}>
                                <input className="form-control" type="text" value={a.asset.name} onChange={e => {
                                    a.asset.name = e.target.value;
                                    setAssets([...assets]);
                                }}/>
                            </td>
                            <td>
                                <input type="range" min="0" max={Math.min(100, a.change + allocationRemaining)} className={styles.slider}
                                    value={a.change}  onChange={e => {
                                    a.change = parseFloat(e.target.value);
                                    setAssets([...assets]);
                                }}/>
                                <span>{a.change.toFixed(2)}%</span>
                            </td>
                            <td>
                                <select value={a.asset._class.typ} className="form-control"  onChange={e => {
                                    a.asset._class.typ = e.target.value as any;
                                    a.asset.annualized_performance = getClass(e.target.value).apy;
                                    setAssets([...assets]);
                                }}>
                                    {classes.map(c => <option key={c._class.typ} value={c._class.typ}>{c._class.typ}</option>)}
                                </select>
                                {
                                    a.asset._class.typ === AssetClassTypEnum.Custom &&
                                        <input type="number" min="0" max="100" className="form-control"
                                               value={getReturn(a.asset.annualized_performance)} onChange={e => {
                                            a.asset.annualized_performance = 1 + (parseFloat(e.target.value) / 100);
                                            console.log(e.target.value,  a.asset.annualized_performance);
                                            setAssets([...assets]);
                                        }}/>
                                }
                                {
                                    a.asset._class.typ !== AssetClassTypEnum.Custom &&
                                    <span>({getReturn(a.asset.annualized_performance).toFixed(1)}%)</span>
                                }
                            </td>
                            <td className={styles.actions}>
                                <i className="fa fa-times" aria-hidden="true" onClick={() => removeAsset(a)}/>
                            </td>
                        </tr>)
                }
                </tbody>
            </table>
            <button className="btn btn-primary" onClick={() => save()}>
                Save Assets
            </button>
        </Modal.Body>
    </Modal>
}
