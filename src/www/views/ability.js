import { useEffect, useState } from "preact/hooks";
import { abilities } from "../../assets/abilities.toml";
import "../styles/ability.css";

/**
 * Component for a single ability's icon, optionally with button binding.
 * @param {{ id: string, binding?: string }} props
 */
export function AbilityIcon({ id, binding, size=64 }) {
    return (
        <div class="ability-icon" style={`width: ${size}px; height: ${size}px;`}>
            <img class="unselectable" src={abilities[id]?.icon}/>
            {binding && (
                <KeyboardBinding letter={binding}/>
            )}
        </div>
    );
}

/**
 * Component that displays 4 `AbilityIcon`'s
 */
export function AbilityDeck({ deck }) {
    console.assert(deck.length === 4);
    return (
        <div class="ability-deck row">
            {deck.map((id, i) => (
                <AbilityIcon id={id} key={i} size={72}/>
            ))}
        </div>
    );
}

/**
 * Component that displays a player's unlocked ability cards.
 */
export function AbilityCollection({ collection }) {
    return (
        <div class="ability-collection">
            {collection.map((id, i) => (
                <AbilityIcon id={id} key={i} size={72}/>
            ))}
            {!collection.length && (
                <div class="centered">
                    No cards unlocked.
                </div>
            )}
        </div>
    );
}

/**
 * Component for a keyboard ability binding.
 * TODO: add a GamepadBinding then generic Binding component.
 */
export function KeyboardBinding({ letter }) {
    const [pressed, setPressed] = useState(false);
    
    // Bind keypress listener.
    useEffect(() => {
        document.addEventListener("keydown", (e) => {
            if (e.key === letter) {
                console.log(e.key);
                setPressed(true);
            }
        });
        document.addEventListener("keyup", (e) => {
            if (e.key === letter) {
                setPressed(false);
            }
        });
    }, []);

    return (
        <div class={"binding-keyboard unselectable " + (pressed ? "pressed" : "")}>
            {letter}
        </div>
    );
}