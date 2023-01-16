import { render } from "preact";
import { useEffect } from "preact/hooks";
import { useWasm } from "./hooks";

import geckos from "@geckos.io/client";
import { port } from "argv";

import lib from "./lib.rs";
import "./styles/root.css";

function App() {
    const rs = useWasm(lib);

    useEffect(() => {
        const channel = geckos({ port });
        
        channel.onConnect((err) => {
            if (err) {
                console.error(err);
            } else {
                console.log("Connected to server!", channel);
            }
        });
    }, []);

    return (
        <div class="centered rows">
            2 + 2 = {rs?.add(2, 2)}
        </div>
    );
}

render(<App/>, document.body);