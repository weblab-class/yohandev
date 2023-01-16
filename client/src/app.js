import { render } from "preact";
import { useWasm } from "./hooks";

import lib from "./lib.rs";
import "./styles/root.css";

function App() {
    const rs = useWasm(lib);

    return (
        <div class="centered rows">
            2 + 2 = {rs?.add(2, 2)}
        </div>
    );
}

render(<App/>, document.body);