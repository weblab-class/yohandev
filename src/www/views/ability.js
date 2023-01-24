import { useEffect, useState } from "preact/hooks";
import { abilities } from "../../assets/abilities.toml";
import "../styles/ability.css";

/**
 * Component for a single ability's icon, optionally with button binding.
 * @param {{ icon: string, binding?: string }} param0
 */
export function AbilityIcon({ id, binding }) {
    return (
        <div class="ability-icon">
            <img src={abilities[id].icon}/>
            {binding && (
                <KeyboardBinding letter={binding}/>
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