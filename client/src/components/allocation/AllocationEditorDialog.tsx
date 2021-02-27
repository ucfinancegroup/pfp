import styles from "./AllocationEditorDialog.module.scss"
import classNames from "classnames";
import React, {useEffect, useState} from "react";
import {AssetClassAndApy, AssetClassesApi} from "../../api";
import Modal from "react-bootstrap/cjs/Modal";

const cx = classNames.bind(styles);

type AllocationEditorDialogProps = {
    show: boolean,
    onClose: (x) => void;
};

const assetsApi = new AssetClassesApi();

export function AllocationEditorDialog(props: AllocationEditorDialogProps) {
    const [error, setError] = useState<string>();
    const [classes, setClasses] = useState<AssetClassAndApy[]>();
    useEffect(() => {
        getClasses();
    }, []);

    async function getClasses() {
        const assets = await assetsApi.getAssetClasses();
        setClasses(assets);
    }


    function close() {
        props.onClose(null);
    }

    function getReturn(apy) {
        return (apy - 1) * 100;
    }

    return <Modal show={props.show} onHide={() => close()}  dialogClassName="modal-large">
        <Modal.Header closeButton>
            <Modal.Title>Edit Allocations</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            <table className="table">
                <thead>
                <tr>
                    <th>
                        Asset Class
                    </th>
                    <th>
                        Annual Return
                    </th>
                    <th>
                        Allocation
                    </th>
                </tr>
                </thead>
                <tbody>
                {
                    classes && classes.map(cls =>
                        <tr key={cls._class.typ}>
                            <td className={styles.class__name}>
                                {cls._class.typ}
                            </td>
                            <td>
                                {getReturn(cls.apy).toFixed(1)}%
                            </td>
                            <td>
                                <input type="range" min="0" max="100" className={styles.slider}/>
                                <span className={styles.allocation}></span>
                            </td>
                        </tr>)
                }
                </tbody>
            </table>
        </Modal.Body>
    </Modal>
}
