import { useEffect, useState, useRef } from "preact/hooks";
import { Sortable, Swap } from "sortablejs/modular/sortable.core.esm";
import { abilities } from "../../assets/abilities.toml";
import "../styles/ability.css";

/**
 * Component for a single ability's icon, optionally with button binding.
 * @param {{ id: string, binding?: string }} props
 */
export function AbilityIcon({ id, binding, size=64, cooldown=0.0, ...props }) {
    const ref = useRef();
    useEffect(() => {
        if (cooldown <= 0.0 || !ref.current) {
            return;
        }
        let k = cooldown * 10;
        let i = k;
        let j = setInterval(() => {
            if ((i -= 1) <= 0) {
                clearInterval(j);
            }
            ref.current.style.filter = `grayscale(${i / k * 100}%)`;
        }, 100);
    }, [cooldown]);
    return (
        <div
            class={`ability-icon unselectable`}
            style={`width: ${size}px; height: ${size}px;`}
            ref={ref}
            onClick={() => binding && alert("Use 1-4 on your keyboard!")}
            {...props}
        >
            <img src={abilities[id]?.icon}/>
            {binding && (
                <KeyboardBinding letter={binding}/>
            )}
        </div>
    );
}

export function AbilityInventory({ deck, collection, onHover, onSwap }) {
    const deckRef = useRef();
    const collectionRef = useRef();

    function AbilityIcon2({ id, i }) {
        return (
            <AbilityIcon
                id={id}
                key={i}
                size={72}
                onMouseOver={(_) => onHover(id)}
                onMouseOut={(_) => onHover(undefined)}
                data-ability={id}
            /> 
        );
    }
    // These effects are needed otherwise preact will re-render this entire
    // component for no reaosn... weird stuff :P
    useEffect(() => {}, [deck]);
    useEffect(() => {}, [collection]);
    useEffect(() => {}, [onHover]);

    // Implement sortable:
    useEffect(() => {
        const opts = {
            group: "ability-inventory",
            swapClass: "ability-icon-drop",
            animation: 150,
            swap: true,
        }
        Sortable.mount(new Swap());
        Sortable.create(deckRef.current, {
            onSort: (e) => {
                setTimeout(() => {
                    onSwap(e.item.dataset.ability, e.swapItem.dataset.ability);
                }, 200);
            },
            ...opts
        });
        Sortable.create(collectionRef.current, { sort: false, ...opts });
    }, [deckRef, collectionRef]);

    return (
        <div class="ability-inventory">
            <div class="ability-deck columns:4" ref={deckRef}>
                {deck.map((id, i) => (
                    <AbilityIcon2 id={id} key={i}/> 
                ))}
            </div>
            <div class="ability-collection">
                {collection.length
                    ? (<h4>Unlocked Cards</h4>)
                    : (
                        <span class="center column" style="translate: 0 50px">
                            <h2>No cards unlocked.</h2>
                            <p>Did you forget to login?</p>
                        </span>
                )}
                <div class="columns:4" ref={collectionRef}>
                    {collection.filter((id) => !deck.includes(id)).map((id, i) => (
                        <AbilityIcon2 id={id} key={i}/> 
                    ))}
                </div>
            </div>
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