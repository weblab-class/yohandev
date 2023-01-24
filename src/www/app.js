import { render } from "preact";
import { Game } from "./views/game";

import "./styles/root.css";
import { Menu } from "./views/menu";

function App() {
    return (
        <div class="overlap">
            <Game/>
            <Menu class="z:100"/>
        </div>
    );
}

render(<App/>, document.body);