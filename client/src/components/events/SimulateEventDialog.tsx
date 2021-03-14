import styles from "./SimulateEventDialog.module.scss"
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {Allocation, AllocationProportion, AssetClassAndApy, AssetClassesApi, AssetClassTypEnum} from "../../api";
import Modal from "react-bootstrap/cjs/Modal";
import {dateAsInputString} from "../../Helpers";
import {epochToDate} from "../recurring/RecurringHelpers";

const cx = classNames.bind(styles);

type SimulateEventDialogProps = {
    show: boolean,
    onClose: () => void;
};

export function SimulateEventDialog(props: SimulateEventDialogProps) {
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

    }

    function close() {
        props.onClose();
    }

    function getReturn(apy) {
        return Math.round((apy - 1) * 100 * 1000) / 1000;

    }

    function addAsset() {

    }

    function removeAsset(asset: AllocationProportion) {

    }

    const allocationTotal = assets.length === 0 ? 0 : assets.map(a => a.proportion).reduce((a, b) => a + b);
    const allocationRemaining = 100 - allocationTotal;

    function save() {

    }


    return <Modal show={props.show} onHide={() => close()}  dialogClassName="modal-large">
        <Modal.Header closeButton>
            <Modal.Title>Simulate Event</Modal.Title>
        </Modal.Header>
        <Modal.Body>

            <button className="btn btn-primary" onClick={() => save()} disabled={allocationRemaining > 0}>
                <i className="fa fa-save"/>
                Save
            </button>
        </Modal.Body>
    </Modal>
}
