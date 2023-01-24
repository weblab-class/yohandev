import { render } from "preact";
import { Game } from "./views/game";

import "./styles/root.css";

function App() {
    return (
        <Game/>
    );
}

render(<App/>, document.body);