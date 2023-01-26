import { useCallback, useState } from "preact/hooks"

import { AbilityCollection, AbilityDeck, AbilityInventory } from "./ability";
import { LoginButton } from "./login";
import { abilities } from "../../assets/abilities.toml";
import { POST } from "../utils";
import "../styles/menu.css";

/**
 * Main menu component.
 */
export function Menu({ ...props }) {
    // User progress
    const [deck, setDeck] = useState(Array(4).fill(undefined));
    const [collection, setCollection] = useState([]);

    const onLogin = useCallback(({ id: _id, deck, unlocked }) => {
        setDeck(deck);
        setCollection(unlocked);
    });
    const onPlay = useCallback(() => POST("/api/join-game"), []);

    return (
        <div {...props}>
            {/* Black overlay */}
            <div
                class="w:100vw h:100vh"
                style="background: rgb(0, 0, 0, 0.2)"
            />
            {/* Floating window */}
            <div class="menu centered column">
                <div class="row">
                    <span class="logo">boxbrawl</span>
                    <LoginButton onLogin={onLogin}/>
                </div>
                <div class="row">
                    <AbilityInventory deck={deck} collection={collection}/>
                    <div class="column">
                        <div class="player-preview">

                        </div>
                        <div class="player-stats">
                            <AbilityDescription id="sniper"/>
                        </div>
                        <button onClick={onPlay}>
                            Play
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}

/**
 * Component that displays player stats given their deck.
 */
function PlayerStats({ deck }) {

}

/**
 * Ability that 
 */
function AbilityDescription({ id }) {
    return (
        <div class="ability-description">
            <h1>{abilities[id].name}</h1>
            <p>{abilities[id].description}</p>
            <div class="ability-stats-container">
                {abilities[id].statistics.map(({ name, value }) => (
                    <>
                        <p>{name}</p>
                        <div class="ability-stats-bar">
                            <div style={`width: ${value / 10}%;`}/>
                        </div>
                    </>
                ))}
            </div>
        </div>
    )
}