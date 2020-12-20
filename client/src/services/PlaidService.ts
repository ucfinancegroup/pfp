export abstract class PlaidService {
    public static async getToken() {
        return (await fetch("/api/plaid/link_token", {
            method: "POST",
        })).json();
    }

    public static async exchangeToken(publicToken: string) {
        return (await fetch("/api/plaid/public_token_exchange", {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                public_token: publicToken,
            })
        })).json();
    }
}
