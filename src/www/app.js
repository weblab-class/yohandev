import { useState } from "preact/hooks";
import { render } from "preact";

import { Game } from "./views/game";
import { Menu } from "./views/menu";

import "./styles/root.css";

function App() {
    const [showMenu, setShowMenu] = useState(true);
    const [deck, setDeck] = useState([]);
    return (
        <div class="overlap">
            <Game deck={deck}/>
            {showMenu && (
                <Menu
                    class="z:100"
                    startGame={(deck) => {
                        setShowMenu(false);
                        setDeck(deck);
                    }}
                />
            )}
        </div>
    );
}

render(<App/>, document.body);