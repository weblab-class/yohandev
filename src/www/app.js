import { useState } from "preact/hooks";
import { render } from "preact";

import { Game } from "./views/game";
import { Menu } from "./views/menu";

import "./styles/root.css";

function App() {
    const [playing, setPlaying] = useState(false);
    const [deck, setDeck] = useState([]);

    return (
        <div class="overlap">
            <Game deck={deck} playing={playing}/>
            {!playing && (
                <Menu
                    class="z:100"
                    startGame={(deck) => {
                        setPlaying(true);
                        setDeck(deck);
                    }}
                />
            )}
        </div>
    );
}

render(<App/>, document.body);