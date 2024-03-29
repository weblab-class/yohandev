import { useCallback, useRef, useState } from "preact/hooks"

import { AbilityInventory } from "./ability";
import { LoginButton } from "./login";
import { abilities } from "../../assets/abilities.toml";
import { POST } from "../utils";
import "../styles/menu.css";

/**
 * Main menu component.
 */
export function Menu({ startGame, ...props }) {
    // User progress
    const [deck, setDeck] = useState([
        // Default deck:
        "shotgun",
        "dual-gun",
        "assault-rifle",
        "shield"
    ]);
    const [collection, setCollection] = useState([]);
    // Dynamic UI stuff
    const [hovered, setHovered] = useState(undefined);
    const ref = useRef();

    const onLogin = useCallback(({ id: _id, deck, unlocked }) => {
        setDeck(deck);
        setCollection(unlocked);
    }, []);
    const onEditDeck = useCallback((added, removed) => {
        setDeck((prev) => prev.map((id) => {
            // Swap into deck
            if (id == removed) {
                return added;
            }
            // Deck-deck reordering
            if (id == added) {
                return removed;
            }
            return id;
        }));
        POST("/api/edit-deck", { added, removed });
    }, []);
    const onPlay = useCallback(() => {
        // Fade-out
        ref.current?.classList.add("fade-out");
        // Disappear
        setTimeout(() => startGame(deck), 150);
    }, [ref, deck]);
    const onHover = useCallback((id) => setHovered(id), []);

    return (
        <div ref={ref} {...props}>
            {/* Black overlay */}
            <div
                class="w:100vw h:100vh"
                style="background: rgb(0, 0, 0, 0.2)"
            />
            {/* Floating window */}
            <div class="menu centered column">
                <div class="nav-bar row">
                    <span class="logo">boxbrawl</span>
                    <LoginButton onLogin={onLogin}/>
                </div>
                <div class="row">
                    <AbilityInventory
                        deck={deck}
                        collection={collection}
                        onHover={onHover}
                        onSwap={onEditDeck}
                    />
                    <div class="column">
                        <div class="player-preview">

                        </div>
                        <div class="player-stats">
                            {hovered
                                ? (<AbilityDescription id={hovered}/>)
                                : (<Tutorial/>)
                            }
                        </div>
                        <button class="play-button" onClick={onPlay}>
                            Join Game
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
function Tutorial() {
    return (
        <div class="tutorial">
            <img src="assets/tutorial.svg"/>
        </div>
    )
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