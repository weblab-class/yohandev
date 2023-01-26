import { useEffect, useState, useRef } from "preact/hooks";
import { Sortable, Swap } from "sortablejs/modular/sortable.core.esm";
import { abilities } from "../../assets/abilities.toml";
import "../styles/ability.css";

/**
 * Component for a single ability's icon, optionally with button binding.
 * @param {{ id: string, binding?: string }} props
 */
export function AbilityIcon({ id, binding, size=64 }) {
    return (
        <div
            class="ability-icon"
            style={`width: ${size}px; height: ${size}px;`}
        >
            <img src={abilities[id]?.icon}/>
            {binding && (
                <KeyboardBinding letter={binding}/>
            )}
        </div>
    );
}

export function AbilityInventory({ deck, collection }) {
    const deckRef = useRef();
    const collectionRef = useRef();

    // Implement sortable:
    useEffect(() => {
        const opts = {
            group: "ability-inventory",
            swapClass: "ability-icon-drop",
            animation: 150,
            swap: true,
        }
        Sortable.mount(new Swap());
        Sortable.create(deckRef.current, opts);
        Sortable.create(collectionRef.current, { sort: false, ...opts });
    }, [deckRef, collectionRef]);

    return (
        <div class="ability-inventory">
            <div class="ability-deck columns:4" ref={deckRef}>
                {deck.map((id, i) => (
                    <AbilityIcon id={id} key={i} size={72}/> 
                ))}
            </div>
            <span class="ability-collection-header">
                
            </span>
            <div class="ability-collection">
                {collection.length ? "Unlocked Cards" : (
                    <span class="center column" style="translate: 0 50px">
                        <h2>No cards unlocked.</h2>
                        <p>Did you forget to login?</p>
                    </span>
                )}
                <div class="columns:4" ref={collectionRef}>
                    {collection.map((id, i) => (
                        <AbilityIcon id={id} key={i} size={72}/> 
                    ))}
                </div>
            </div>
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
                <div>
                    <AbilityIcon id={id} key={i} size={72}/>
                </div>
            ))}
        </div>
    );
}

/**
 * Component that displays a player's unlocked ability cards.
 */
export function AbilityCollection({ collection }) {
    function onDrop(e) {
        console.log("DROP");
    }
    function allowDrop(e) {
        e.preventDefault();
    }
    return (
        <div class="ability-collection">
            {collection.map((id, i) => (
                <div draggable="true" onDrop={onDrop} onDragOver={allowDrop}>
                    <AbilityIcon id={id} key={i} size={72}/>
                </div>
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