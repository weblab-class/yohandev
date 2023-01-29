import { useEffect, useRef, useMemo } from "preact/hooks";
import { game } from "../../platform/web";
import { AbilityIcon } from "./ability";

import { port } from "env";

import "../styles/game.css";

export function Game({ deck }) {
    const ref = useRef();
    const instance = useMemo(() => game(port), []);

    // Attach the instance of the game to DOM:
    useEffect(() => {
        instance.then(({ hook }) => {
            hook(ref.current);
        });
    }, [ref]);

    return (
        <div ref={ref} class="w:100vw h:100vh">
            <div class="ability-bar row">
                {deck.map((ability, i) => (
                    <AbilityIcon id={ability} binding={i + 1} key={i}/>    
                ))}
            </div>
        </div>
    );
}