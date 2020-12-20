import React, { useCallback } from 'react';
import { usePlaidLink } from 'react-plaid-link';
import Button from "react-bootstrap/cjs/Button";
import {PlaidService} from "../../services/PlaidService";

type PlaidLinkProps = {
    token: string,
}

export default function PlaidLink(props: PlaidLinkProps) {
    const onSuccess = useCallback(
        async (token, metadata) => {
            console.log('onSuccess', token, metadata);
            await PlaidService.exchangeToken(token);
        },
        []
    );

    const onEvent = useCallback(
        (eventName, metadata) => console.log('onEvent', eventName, metadata),
        []
    );

    const onExit = useCallback(
        (err, metadata) => console.log('onExit', err, metadata),
        []
    );

    const config = {
        token: props.token,
        onSuccess,
        onEvent,
        onExit,
        // –– optional parameters
        // receivedRedirectUri: props.receivedRedirectUri || null,
        // ...
    };

    const { open, ready, error } = usePlaidLink(config);

    return (
        <Button onClick={() => open()} disabled={!ready}>
            Connect a bank account
        </Button>
    );
};
