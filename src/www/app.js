import { useState } from "preact/hooks";
import { render } from "preact";

import { Game } from "./views/game";
import { Menu } from "./views/menu";

import "./styles/root.css";

function App() {
    const [showMenu, setShowMenu] = useState(true);
    return (
        <div class="overlap">
            <Game/>
            {showMenu && (
                // TODO: startGame callback will pass user's abilities to game
                <Menu class="z:100" startGame={() => setShowMenu(false)}/>
            )}
        </div>
    );
}

render(<App/>, document.body);