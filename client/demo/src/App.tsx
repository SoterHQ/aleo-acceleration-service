import React from "react";
import { Client, TransferParams } from "../../src";
export default function App() {

    function isWebsite(str: string) {
        try {
            new URL(str);
            return true;
        } catch (error) {
            return false;
        }
    }

    async function handleSubmit(event: any) {
        event.preventDefault();
        const form = event.target;
        const formData = new FormData(form);
        // Or you can work with it as a plain object:
        const formJson = Object.fromEntries(formData.entries());
        const server = formJson.myInput as string;
        if (server && isWebsite(server)) {
            const url = new URL(server);
            console.log(url.username);
            let client = await initClient(url);
            console.log(client);
            // TODO:
            // updateTest(client);
            transferTest(client);
        } else {
            alert("server error!!")
        }
    }

    async function initClient(url: URL) {
        return await Client.new(url);
    }


    async function transferTest(client: Client) {
        let transfer: TransferParams = {
            private_key: 'APrivateKey1zkp31UnRUkKRWCFeKQDf1N9CV9uZxuGtpnrQPRbKRtaFrzW',
            recipient: 'aleo17ha7zdps004mphrg9den6n73fn6rvsartqn49qmxrs3m37egnggs8kkfhj',
            amount: 100000000,
            function: 'public',
        };
        let resp = await client.transfer(transfer);

        console.log(resp);
    }

    async function updateTest(client: Client) {
        let resp = await client.update({ version: "1/.11" });

        console.log(resp);
    }

    return (<form method="post" onSubmit={handleSubmit}>
        <label>
            Server URL: <input name="myInput" />
        </label>
        <hr />
        <button type="submit">Client</button>
    </form>)
}