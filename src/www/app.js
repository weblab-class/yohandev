import { render } from "preact";
import { Game } from "./views/game";

import { Menu } from "./views/menu";
import "./styles/root.css";

function App() {
    return (
        <div class="overlap">
            <Game/>
            <Menu class="z:100"/>
        </div>
    );
}

render(<App/>, document.body);